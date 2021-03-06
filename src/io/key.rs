#[cfg(feature = "orbital")]
use orbclient::{
    KeyEvent,
    K_BKSP,
    K_LEFT,
    K_RIGHT,
    K_UP,
    K_DOWN,
    K_TAB,
    K_ESC,
};

#[derive(Copy, Clone, PartialEq)]
/// A key
pub enum Key {
    /// Printable character.
    Char(char),
    // TODO: Space modifier?
    /// Backspace.
    Backspace,
    /// Escape.
    Escape,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Tab.
    Tab,
    /// Null/unknown key.
    Null,
    /// Quit (close the window).
    Quit,
    /// Unknown key.
    Unknown(u8),
}

impl Key {
    /// Convern an Orbital key event to a `Key`.
    #[cfg(feature = "orbital")]
    pub fn from_event(k: KeyEvent) -> Key {
        match k.character {
            '\0' => match k.scancode {
                s if k.pressed => match s {
                    K_BKSP => Key::Backspace,
                    K_LEFT => Key::Left,
                    K_RIGHT => Key::Right,
                    K_UP => Key::Up,
                    K_DOWN => Key::Down,
                    K_TAB => Key::Tab,
                    K_ESC => Key::Escape,
                    _ => Key::Unknown(s),

                },
                _ => Key::Null,
            },
            c => Key::Char(c),
        }
    }

    /// Convert a `Key` to its corresponding character. If no corresponding character exists, use
    /// the null character.
    pub fn to_char(self) -> char {
        match self {
            Key::Char(c) => c,
            _ => '\0',
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// A command, i.e. a key together with information on the modifiers.
pub struct Cmd {
    /// The key associated with the command.
    pub key: Key,
}
