use crate::sequence::{KeyCode, Sequence, SequenceTerminator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyType {
    Unicode(char),
    Functional(FunctionalKey),
    #[default]
    Unknown,
}

impl<'a> KeyType {
    pub fn to_sequence(&self) -> Option<Sequence<'a>> {
        match self {
            KeyType::Unicode(ch) => Some(Sequence {
                key_code: KeyCode {
                    key_code: (*ch).into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            KeyType::Functional(func) => Some(func.to_sequence()),
            KeyType::Unknown => None,
        }
    }

    pub fn to_key_code(&self) -> Option<u32> {
        match self {
            KeyType::Unicode(ch) => Some((*ch).into()),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionalKey {
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Left,
    Right,
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,

    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,

    NumPadDecimal,
    NumPadDivide,
    NumPadMultply,
    NumPadSubtract,
    NumPadAdd,
    NumPadEnter,
    NumPadEqual,
    NumPadSeparator,
    NumPadLeft,
    NumPadRight,
    NumPadUp,
    NumPadDown,
    NumPadPageUp,
    NumPadPageDown,
    NumPadHome,
    NumPadEnd,
    NumPadInsert,
    NumPadDelete,
    NumPadBegin,

    MediaPlay,
    MediaPause,
    MediaPlayPause,
    MediaReverse,
    MediaStop,
    MediaFastForward,
    MediaRewind,
    MediaTrackNext,
    MediaTrackPrevious,
    MediaRecord,

    LowerVolume,
    RaiseVolume,
    MuteVolume,

    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    LeftHyper,
    LeftMeta,

    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    RightHyper,
    RightMeta,

    IsoLevel3Shift,
    IsoLevel5Shift,
}

impl<'a> FunctionalKey {
    pub fn to_sequence(self) -> Sequence<'a> {
        macro_rules! seq {
            ($num:expr) => {
                Sequence {
                    key_code: KeyCode {
                        key_code: $num,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            };

            ($num:expr, $ch:literal) => {
                Sequence {
                    key_code: KeyCode {
                        key_code: $num,
                        ..Default::default()
                    },
                    terminator: SequenceTerminator::Other($ch),
                    ..Default::default()
                }
            };
        }

        match self {
            FunctionalKey::Escape => seq!(27),
            FunctionalKey::Enter => seq!(13),
            FunctionalKey::Tab => seq!(9),
            FunctionalKey::Backspace => seq!(127),
            FunctionalKey::Insert => seq!(2, '~'),
            FunctionalKey::Delete => seq!(3, '~'),
            FunctionalKey::Left => seq!(1, 'D'),
            FunctionalKey::Right => seq!(1, 'C'),
            FunctionalKey::Up => seq!(1, 'A'),
            FunctionalKey::Down => seq!(1, 'B'),
            FunctionalKey::PageUp => seq!(5, '~'),
            FunctionalKey::PageDown => seq!(6, '~'),
            FunctionalKey::Home => seq!(1, 'H'),
            FunctionalKey::End => seq!(1, 'F'),
            FunctionalKey::CapsLock => seq!(57358),
            FunctionalKey::ScrollLock => seq!(57359),
            FunctionalKey::NumLock => seq!(57360),
            FunctionalKey::PrintScreen => seq!(57361),
            FunctionalKey::Pause => seq!(57362),
            FunctionalKey::Menu => seq!(57363),

            FunctionalKey::F1 => seq!(1, 'P'),
            FunctionalKey::F2 => seq!(1, 'Q'),
            FunctionalKey::F3 => seq!(13, '~'),
            FunctionalKey::F4 => seq!(1, 'S'),
            FunctionalKey::F5 => seq!(15, '~'),
            FunctionalKey::F6 => seq!(17, '~'),
            FunctionalKey::F7 => seq!(18, '~'),
            FunctionalKey::F8 => seq!(19, '~'),
            FunctionalKey::F9 => seq!(20, '~'),
            FunctionalKey::F10 => seq!(21, '~'),
            FunctionalKey::F11 => seq!(23, '~'),
            FunctionalKey::F12 => seq!(24, '~'),
            FunctionalKey::F13 => seq!(57376),
            FunctionalKey::F14 => seq!(57377),
            FunctionalKey::F15 => seq!(57378),
            FunctionalKey::F16 => seq!(57379),
            FunctionalKey::F17 => seq!(57380),
            FunctionalKey::F18 => seq!(57381),
            FunctionalKey::F19 => seq!(57382),
            FunctionalKey::F20 => seq!(57383),
            FunctionalKey::F21 => seq!(57384),
            FunctionalKey::F22 => seq!(57385),
            FunctionalKey::F23 => seq!(57386),
            FunctionalKey::F24 => seq!(57387),
            FunctionalKey::F25 => seq!(57388),
            FunctionalKey::F26 => seq!(57389),
            FunctionalKey::F27 => seq!(57390),
            FunctionalKey::F28 => seq!(57391),
            FunctionalKey::F29 => seq!(57392),
            FunctionalKey::F30 => seq!(57393),
            FunctionalKey::F31 => seq!(57394),
            FunctionalKey::F32 => seq!(57395),
            FunctionalKey::F33 => seq!(57396),
            FunctionalKey::F34 => seq!(57397),
            FunctionalKey::F35 => seq!(57398),

            FunctionalKey::NumPad0 => seq!(57399),
            FunctionalKey::NumPad1 => seq!(57400),
            FunctionalKey::NumPad2 => seq!(57401),
            FunctionalKey::NumPad3 => seq!(57402),
            FunctionalKey::NumPad4 => seq!(57403),
            FunctionalKey::NumPad5 => seq!(57404),
            FunctionalKey::NumPad6 => seq!(57405),
            FunctionalKey::NumPad7 => seq!(57406),
            FunctionalKey::NumPad8 => seq!(57407),
            FunctionalKey::NumPad9 => seq!(57408),

            FunctionalKey::NumPadDecimal => seq!(57409),
            FunctionalKey::NumPadDivide => seq!(57410),
            FunctionalKey::NumPadMultply => seq!(57411),
            FunctionalKey::NumPadSubtract => seq!(57412),
            FunctionalKey::NumPadAdd => seq!(57413),
            FunctionalKey::NumPadEnter => seq!(57414),
            FunctionalKey::NumPadEqual => seq!(57415),
            FunctionalKey::NumPadSeparator => seq!(57416),
            FunctionalKey::NumPadLeft => seq!(57417),
            FunctionalKey::NumPadRight => seq!(57418),
            FunctionalKey::NumPadUp => seq!(57419),
            FunctionalKey::NumPadDown => seq!(57420),
            FunctionalKey::NumPadPageUp => seq!(57421),
            FunctionalKey::NumPadPageDown => seq!(57422),
            FunctionalKey::NumPadHome => seq!(57423),
            FunctionalKey::NumPadEnd => seq!(57424),
            FunctionalKey::NumPadInsert => seq!(57425),
            FunctionalKey::NumPadDelete => seq!(57426),
            FunctionalKey::NumPadBegin => seq!(1, 'E'),

            FunctionalKey::MediaPlay => seq!(57428),
            FunctionalKey::MediaPause => seq!(57429),
            FunctionalKey::MediaPlayPause => seq!(57430),
            FunctionalKey::MediaReverse => seq!(57431),
            FunctionalKey::MediaStop => seq!(57432),
            FunctionalKey::MediaFastForward => seq!(57433),
            FunctionalKey::MediaRewind => seq!(57434),
            FunctionalKey::MediaTrackNext => seq!(57435),
            FunctionalKey::MediaTrackPrevious => seq!(57436),
            FunctionalKey::MediaRecord => seq!(57437),

            FunctionalKey::LowerVolume => seq!(57438),
            FunctionalKey::RaiseVolume => seq!(57439),
            FunctionalKey::MuteVolume => seq!(57440),

            FunctionalKey::LeftShift => seq!(57441),
            FunctionalKey::LeftControl => seq!(57442),
            FunctionalKey::LeftAlt => seq!(57443),
            FunctionalKey::LeftSuper => seq!(57444),
            FunctionalKey::LeftHyper => seq!(57445),
            FunctionalKey::LeftMeta => seq!(57446),

            FunctionalKey::RightShift => seq!(57447),
            FunctionalKey::RightControl => seq!(57448),
            FunctionalKey::RightAlt => seq!(57449),
            FunctionalKey::RightSuper => seq!(57450),
            FunctionalKey::RightHyper => seq!(57451),
            FunctionalKey::RightMeta => seq!(57452),

            FunctionalKey::IsoLevel3Shift => seq!(57453),
            FunctionalKey::IsoLevel5Shift => seq!(57454),
        }
    }

    pub fn is_numpad(&self) -> bool {
        matches!(
            self,
            FunctionalKey::NumPad0
                | FunctionalKey::NumPad1
                | FunctionalKey::NumPad2
                | FunctionalKey::NumPad3
                | FunctionalKey::NumPad4
                | FunctionalKey::NumPad5
                | FunctionalKey::NumPad6
                | FunctionalKey::NumPad7
                | FunctionalKey::NumPad8
                | FunctionalKey::NumPad9
                | FunctionalKey::NumPadDecimal
                | FunctionalKey::NumPadDivide
                | FunctionalKey::NumPadMultply
                | FunctionalKey::NumPadSubtract
                | FunctionalKey::NumPadAdd
                | FunctionalKey::NumPadEnter
                | FunctionalKey::NumPadEqual
                | FunctionalKey::NumPadSeparator
                | FunctionalKey::NumPadLeft
                | FunctionalKey::NumPadRight
                | FunctionalKey::NumPadUp
                | FunctionalKey::NumPadDown
                | FunctionalKey::NumPadPageUp
                | FunctionalKey::NumPadPageDown
                | FunctionalKey::NumPadHome
                | FunctionalKey::NumPadEnd
                | FunctionalKey::NumPadInsert
                | FunctionalKey::NumPadDelete
                | FunctionalKey::NumPadBegin
        )
    }

    pub fn legacy_representation(&self) -> Option<&'static str> {
        Some(match self {
            FunctionalKey::Escape => "\x1b",
            FunctionalKey::Enter => "\r",
            FunctionalKey::Tab => "\t",
            FunctionalKey::Backspace => "\x08",
            _ => return None,
        })
    }
}
