#![no_std]

use core::fmt::{Display, Write};

use bitflags::bitflags;
use key::{FunctionalKey, KeyType};
use sequence::{AssociatedText, EventType, KeyboardModifiers, Sequence};

pub mod key;
pub mod sequence;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct ReportingMode: u8 {
        const DISAMBIGUATE_ESC_CODES  = 0b0000_0001;
        const REPORT_EVENT_TYPES      = 0b0000_0010;
        const REPORT_ALTERNATE_KEYS   = 0b0000_0100;
        const REPORT_ALL_KEYS_AS_ESC  = 0b0000_1000;
        const REPORT_ASSOCIATED_TEXT  = 0b0001_0000;
    }
}

#[derive(Debug, Clone, Default)]
pub enum EventResponse<'a> {
    Text {
        text: &'a str,
        alt_pressed: bool,
    },
    Character {
        character: char,
        alt_pressed: bool,
    },
    Sequence(Sequence<'a>),
    #[default]
    Nothing,
}

impl<'a> Display for EventResponse<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EventResponse::Text {
                text,
                alt_pressed: false,
            } => f.write_str(text),
            EventResponse::Text {
                text,
                alt_pressed: true,
            } => {
                write!(f, "\x1b{text}")
            }
            EventResponse::Character {
                character,
                alt_pressed: false,
            } => f.write_char(*character),
            EventResponse::Character {
                character,
                alt_pressed: true,
            } => {
                write!(f, "\x1b{character}")
            }
            EventResponse::Sequence(seq) => seq.fmt(f),
            EventResponse::Nothing => Ok(()),
        }
    }
}

pub trait KeyEvent {
    fn key_with_modifiers(&self) -> KeyType;
    fn key_without_modifiers(&self) -> KeyType;
    fn key_base_layout(&self) -> KeyType;

    fn modifiers(&self) -> KeyboardModifiers;
    fn event_type(&self) -> EventType;
    fn associated_text(&self) -> Option<AssociatedText>;
}

pub fn generate_sequence(mode: ReportingMode, key_event: &impl KeyEvent) -> EventResponse {
    let shifted_key = key_event.key_with_modifiers();
    let unshifted_key = key_event.key_without_modifiers();
    let modifiers = key_event.modifiers();

    // Don't report release event when event types are not reported
    if key_event.event_type() == EventType::Release
        && !mode.intersects(ReportingMode::REPORT_EVENT_TYPES)
    {
        return EventResponse::Nothing;
    }

    let response = if mode.intersects(ReportingMode::REPORT_ALL_KEYS_AS_ESC) {
        if let Some(sequence) = unshifted_key.to_sequence() {
            EventResponse::Sequence(sequence)
        } else {
            EventResponse::Nothing
        }
    } else if mode.intersects(ReportingMode::DISAMBIGUATE_ESC_CODES) {
        let exception = matches!(
            unshifted_key,
            KeyType::Functional(
                FunctionalKey::Enter | FunctionalKey::Tab | FunctionalKey::Backspace
            )
        );

        if modifiers.difference(KeyboardModifiers::SHIFT).is_empty() || exception {
            match shifted_key {
                KeyType::Functional(func @ FunctionalKey::Escape) => {
                    EventResponse::Sequence(func.to_sequence())
                }
                KeyType::Functional(func) if func.is_numpad() => {
                    EventResponse::Sequence(func.to_sequence())
                }
                KeyType::Functional(func) => match func.legacy_representation() {
                    Some(repr) => EventResponse::Text {
                        text: repr,
                        alt_pressed: false,
                    },
                    None => EventResponse::Sequence(func.to_sequence()),
                },
                KeyType::Unicode(character) => EventResponse::Character {
                    character,
                    alt_pressed: false,
                },
                KeyType::Unknown => EventResponse::Nothing,
            }
        } else if let Some(sequence) = unshifted_key.to_sequence() {
            EventResponse::Sequence(sequence)
        } else {
            EventResponse::Nothing
        }
    } else {
        match shifted_key {
            KeyType::Unicode(character) => EventResponse::Character {
                character,
                alt_pressed: modifiers.intersects(KeyboardModifiers::ALT),
            },
            KeyType::Functional(func) => {
                if let Some(text) = func
                    .legacy_representation()
                    .or_else(|| key_event.associated_text().map(|at| at.0))
                {
                    EventResponse::Text {
                        text,
                        alt_pressed: modifiers.intersects(KeyboardModifiers::ALT),
                    }
                } else {
                    EventResponse::Sequence(func.to_sequence())
                }
            }
            KeyType::Unknown => EventResponse::Nothing,
        }
    };

    match response {
        EventResponse::Sequence(mut sequence) => {
            sequence.modifier = modifiers;

            if mode.intersects(ReportingMode::REPORT_EVENT_TYPES) {
                sequence.event_type = key_event.event_type();
            }

            if mode.intersects(ReportingMode::REPORT_ALTERNATE_KEYS) {
                if modifiers.intersects(KeyboardModifiers::SHIFT) {
                    sequence.key_code.shifted_key_code = shifted_key.to_key_code();
                }
                sequence.key_code.base_layout_key_code = key_event.key_base_layout().to_key_code();
            }

            if mode.intersects(ReportingMode::REPORT_ASSOCIATED_TEXT) {
                sequence.associated_text = key_event.associated_text();
            }

            EventResponse::Sequence(sequence)
        }
        _ if key_event.event_type() == EventType::Release => EventResponse::Nothing,
        a => a,
    }
}

#[cfg(test)]
mod tests {

    use super::sequence::*;
    use super::*;

    extern crate std;

    use std::format;
    use std::string::String;

    #[test]
    fn response_display() {
        let short_sequence = Sequence {
            key_code: KeyCode {
                key_code: u32::from('a'),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            format!("{}", EventResponse::Sequence(short_sequence)),
            "\x1b[97u"
        );

        assert_eq!(
            format!(
                "{}",
                EventResponse::Text {
                    text: "a",
                    alt_pressed: false
                }
            ),
            "a"
        );
        assert_eq!(
            format!(
                "{}",
                EventResponse::Text {
                    text: "a",
                    alt_pressed: true
                }
            ),
            "\x1ba"
        );
    }

    #[derive(Debug, Clone, Default)]
    struct DummyKeyEvent {
        key_with_modifiers: KeyType,
        key_without_modifiers: KeyType,
        key_base_layout: KeyType,

        modifiers: KeyboardModifiers,
        event_type: EventType,
        associated_text: Option<String>,
    }

    impl KeyEvent for DummyKeyEvent {
        fn key_with_modifiers(&self) -> KeyType {
            self.key_with_modifiers
        }

        fn key_without_modifiers(&self) -> KeyType {
            self.key_without_modifiers
        }

        fn key_base_layout(&self) -> KeyType {
            self.key_base_layout
        }

        fn modifiers(&self) -> KeyboardModifiers {
            self.modifiers
        }

        fn event_type(&self) -> EventType {
            self.event_type
        }

        fn associated_text(&self) -> Option<AssociatedText> {
            self.associated_text.as_ref().map(|t| AssociatedText(&t))
        }
    }

    macro_rules! generation_test {
        ($fn_name:ident, $mode:expr, $shifted:literal, $escape:literal, $backspace:literal, $arrow:literal, $numpad:literal, $ctrl_c:literal, $release:literal) => {
            #[test]
            fn $fn_name() {
                let mode = $mode;

                let unicode_event = DummyKeyEvent {
                    key_with_modifiers: KeyType::Unicode('A'),
                    key_without_modifiers: KeyType::Unicode('a'),

                    modifiers: KeyboardModifiers::SHIFT,
                    ..Default::default()
                };

                let response = generate_sequence(mode, &unicode_event);

                assert_eq!(format!("{response}"), $shifted, "Shifted A");

                let esc_event = DummyKeyEvent {
                    key_with_modifiers: KeyType::Functional(FunctionalKey::Escape),
                    key_without_modifiers: KeyType::Functional(FunctionalKey::Escape),
                    ..Default::default()
                };

                let response = generate_sequence(mode, &esc_event);

                assert_eq!(format!("{response}"), $escape, "Escape");

                let backspace_event = DummyKeyEvent {
                    key_with_modifiers: KeyType::Functional(FunctionalKey::Backspace),
                    key_without_modifiers: KeyType::Functional(FunctionalKey::Backspace),
                    ..Default::default()
                };

                let response = generate_sequence(mode, &backspace_event);

                assert_eq!(format!("{response}"), $backspace, "Backspace");

                let arrow_event = DummyKeyEvent {
                    key_with_modifiers: KeyType::Functional(FunctionalKey::Up),
                    key_without_modifiers: KeyType::Functional(FunctionalKey::Up),
                    event_type: EventType::Repeat,
                    ..Default::default()
                };

                let response = generate_sequence(mode, &arrow_event);

                assert_eq!(format!("{response}"), $arrow, "Arrow Key Up Released");

                let numpad_event = DummyKeyEvent {
                    key_with_modifiers: KeyType::Functional(FunctionalKey::NumPad5),
                    key_without_modifiers: KeyType::Functional(FunctionalKey::NumPad5),
                    associated_text: Some("5".into()),
                    ..Default::default()
                };

                let response = generate_sequence(mode, &numpad_event);

                assert_eq!(format!("{response}"), $numpad, "NumPad Key 5");

                let ctrl_c_event = DummyKeyEvent {
                    key_with_modifiers: KeyType::Unicode('\x03'),
                    key_without_modifiers: KeyType::Unicode('c'),

                    modifiers: KeyboardModifiers::CTRL,
                    ..Default::default()
                };

                let response = generate_sequence(mode, &ctrl_c_event);

                assert_eq!(format!("{response}"), $ctrl_c, "CTRL + C");

                let release_event = DummyKeyEvent {
                    key_with_modifiers: KeyType::Unicode('b'),
                    key_without_modifiers: KeyType::Unicode('b'),

                    event_type: EventType::Release,
                    ..Default::default()
                };

                let response = generate_sequence(mode, &release_event);

                assert_eq!(format!("{response}"), $release, "Key b released");
            }
        };
    }

    generation_test!(
        test_generation_legacy,
        ReportingMode::empty(),
        "A",
        "\x1b",
        "\x08",
        "\x1b[A",
        "5",
        "\x03",
        ""
    );

    generation_test!(
        test_generation_disambiguate,
        ReportingMode::DISAMBIGUATE_ESC_CODES,
        "A",
        "\x1b[27u",
        "\x08",
        "\x1b[A",
        "\x1b[57404u",
        "\x1b[99;5u",
        ""
    );

    generation_test!(
        test_generation_event_types,
        ReportingMode::DISAMBIGUATE_ESC_CODES | ReportingMode::REPORT_EVENT_TYPES,
        "A",
        "\x1b[27u",
        "\x08",
        "\x1b[;1:2A",
        "\x1b[57404u",
        "\x1b[99;5u",
        ""
    );

    generation_test!(
        test_generation_alternate_keys,
        ReportingMode::DISAMBIGUATE_ESC_CODES
            | ReportingMode::REPORT_EVENT_TYPES
            | ReportingMode::REPORT_ALTERNATE_KEYS,
        "A",
        "\x1b[27u",
        "\x08",
        "\x1b[;1:2A",
        "\x1b[57404u",
        "\x1b[99;5u",
        ""
    );

    generation_test!(
        test_generation_all_keys_as_esc,
        ReportingMode::DISAMBIGUATE_ESC_CODES
            | ReportingMode::REPORT_EVENT_TYPES
            | ReportingMode::REPORT_ALTERNATE_KEYS
            | ReportingMode::REPORT_ALL_KEYS_AS_ESC,
        "\x1b[97:65;2u",
        "\x1b[27u",
        "\x1b[127u",
        "\x1b[;1:2A",
        "\x1b[57404u",
        "\x1b[99;5u",
        "\x1b[98;1:3u"
    );

    generation_test!(
        test_generation_all_modes,
        ReportingMode::all(),
        "\x1b[97:65;2u",
        "\x1b[27u",
        "\x1b[127u",
        "\x1b[;1:2A",
        "\x1b[57404;;53u",
        "\x1b[99;5u",
        "\x1b[98;1:3u"
    );
}
