use core::fmt::{Display, Write};

use bitflags::bitflags;

#[derive(Debug, Clone, Default)]
pub struct Sequence<'a> {
    pub introducer: SequenceIntroducer,
    pub key_code: KeyCode,
    pub modifier: KeyboardModifiers,
    pub event_type: EventType,
    pub associated_text: Option<AssociatedText<'a>>,
    pub terminator: SequenceTerminator,
}

impl<'a> Display for Sequence<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_assert!(
            !self.modifier.intersects(KeyboardModifiers::SHIFT)
                || self.key_code.shifted_key_code.is_some(),
            "KeyCode: {:?}, Modifier: {:?}",
            self.key_code,
            self.modifier
        );

        write!(f, "{}{}", self.introducer, self.key_code)?;

        match (
            self.modifier.is_empty(),
            self.event_type,
            &self.associated_text,
        ) {
            (true, EventType::Press, None) => Ok(()),
            (false, EventType::Press, None) => write!(f, ";{}", self.modifier),
            (true, EventType::Press, Some(associated)) => write!(f, ";;{associated}"),
            (_, _, None) => write!(f, ";{}:{}", self.modifier, self.event_type),
            (false, EventType::Press, Some(associated)) => {
                write!(f, ";{};{associated}", self.modifier)
            }
            (_, _, Some(associated)) => {
                write!(f, ";{}:{};{associated}", self.modifier, self.event_type)
            }
        }?;

        write!(f, "{}", self.terminator)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SequenceIntroducer {
    #[default]
    CSI,
    SS3,
}

impl Display for SequenceIntroducer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CSI => write!(f, "\x1b["),
            Self::SS3 => write!(f, "\x1bO"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyCode {
    pub key_code: u32,
    pub shifted_key_code: Option<u32>,
    pub base_layout_key_code: Option<u32>,
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (
            self.key_code,
            self.shifted_key_code,
            self.base_layout_key_code,
        ) {
            (1, None, None) => Ok(()),
            (_, None, None) => write!(f, "{}", self.key_code),
            (_, Some(alternate), None) => write!(f, "{}:{}", self.key_code, alternate),
            (_, None, Some(base)) => write!(f, "{}::{}", self.key_code, base),
            (_, Some(alternate), Some(base)) => {
                write!(f, "{}:{}:{}", self.key_code, alternate, base)
            }
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct KeyboardModifiers: u8 {
        const SHIFT     = 0b0000_0001;
        const ALT       = 0b0000_0010;
        const CTRL      = 0b0000_0100;
        const SUPER     = 0b0000_1000;
        const HYPER     = 0b0001_0000;
        const META      = 0b0010_0000;
        const CAPS_LOCK = 0b0100_0000;
        const NUM_LOCK  = 0b1000_0000;
    }
}

impl Display for KeyboardModifiers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits() as u16 + 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventType {
    #[default]
    Press,
    Repeat,
    Release,
}

impl Display for EventType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EventType::Press => f.write_char('1'),
            EventType::Repeat => f.write_char('2'),
            EventType::Release => f.write_char('3'),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AssociatedText<'a>(pub &'a str);

impl<'a> Display for AssociatedText<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut chars = self.0.chars();
        if let Some(ch) = chars.next() {
            write!(f, "{}", u32::from(ch))?;
        }
        for ch in chars {
            write!(f, ":{}", u32::from(ch))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SequenceTerminator {
    #[default]
    Kitty,
    Other(char),
}

impl Display for SequenceTerminator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SequenceTerminator::Other(char) => f.write_char(*char),
            SequenceTerminator::Kitty => f.write_char('u'),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    extern crate std;
    use std::format;

    #[test]
    fn introducer_display() {
        assert_eq!(format!("{}", SequenceIntroducer::CSI), "\x1b[");
        assert_eq!(format!("{}", SequenceIntroducer::SS3), "\x1bO");
    }

    #[test]
    fn key_code_display() {
        assert_eq!(
            format!(
                "{}",
                KeyCode {
                    key_code: 1,
                    ..Default::default()
                }
            ),
            ""
        );
        assert_eq!(
            format!(
                "{}",
                KeyCode {
                    key_code: 123,
                    ..Default::default()
                }
            ),
            "123"
        );
        assert_eq!(
            format!(
                "{}",
                KeyCode {
                    key_code: 123,
                    shifted_key_code: Some(456),
                    base_layout_key_code: None
                }
            ),
            "123:456"
        );
        assert_eq!(
            format!(
                "{}",
                KeyCode {
                    key_code: 123,
                    shifted_key_code: None,
                    base_layout_key_code: Some(789)
                }
            ),
            "123::789"
        );
        assert_eq!(
            format!(
                "{}",
                KeyCode {
                    key_code: 123,
                    shifted_key_code: Some(456),
                    base_layout_key_code: Some(789)
                }
            ),
            "123:456:789"
        );
    }

    #[test]
    fn modifiers_display() {
        assert_eq!(format!("{}", KeyboardModifiers::empty()), "1");
        assert_eq!(
            format!(
                "{}",
                KeyboardModifiers::HYPER | KeyboardModifiers::SHIFT | KeyboardModifiers::CTRL
            ),
            "22"
        );
        assert_eq!(format!("{}", KeyboardModifiers::all()), "256");
    }

    #[test]
    fn event_type_display() {
        assert_eq!(format!("{}", EventType::Press), "1");
        assert_eq!(format!("{}", EventType::Repeat), "2");
        assert_eq!(format!("{}", EventType::Release), "3");
    }

    #[test]
    fn associated_text_display() {
        assert_eq!(format!("{}", AssociatedText("")), "");
        assert_eq!(format!("{}", AssociatedText("a")), "97");
        assert_eq!(format!("{}", AssociatedText("abc")), "97:98:99");
    }

    #[test]
    fn terminator_display() {
        assert_eq!(format!("{}", SequenceTerminator::Kitty), "u");
        assert_eq!(format!("{}", SequenceTerminator::Other('~')), "~");
    }

    #[test]
    fn sequence_display() {
        let sequence = Sequence {
            introducer: SequenceIntroducer::CSI,
            key_code: KeyCode {
                key_code: 123,
                shifted_key_code: Some(456),
                base_layout_key_code: Some(789),
            },
            modifier: KeyboardModifiers::HYPER | KeyboardModifiers::SHIFT | KeyboardModifiers::CTRL,
            event_type: EventType::Release,
            associated_text: Some(AssociatedText("abc")),
            terminator: SequenceTerminator::Other('~'),
        };

        assert_eq!(format!("{sequence}"), "\x1b[123:456:789;22:3;97:98:99~");

        let short_sequence = Sequence {
            key_code: KeyCode {
                key_code: u32::from('a'),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(format!("{short_sequence}"), "\x1b[97u");

        let one_based_sequence = Sequence {
            key_code: KeyCode {
                key_code: 1,
                ..Default::default()
            },
            terminator: SequenceTerminator::Other('H'),
            ..Default::default()
        };

        assert_eq!(format!("{one_based_sequence}"), "\x1b[H");
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn invalid_sequence_display_no_alternate() {
        let no_alternate_sequence = Sequence {
            key_code: KeyCode {
                key_code: 'a'.into(),
                ..Default::default()
            },
            modifier: KeyboardModifiers::SHIFT,
            ..Default::default()
        };

        let _ = format!("{no_alternate_sequence}");
    }
}
