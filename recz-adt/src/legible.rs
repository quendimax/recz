use core::fmt::Write;
use owo_colors::OwoColorize;

/// A trait for types interpreted as symbols in a finite automaton. Just has an
/// additional method for formatting the symbol more human friendly.
pub trait Legible {
    /// Returns a wrapper for symbol that can be used for more human legible
    /// formatting.
    fn legible(&self) -> impl core::fmt::Display;

    /// Returns a wrapper for symbol that can be used for more human legible
    /// formatting with color.
    fn colored(&self) -> impl core::fmt::Display;
}

impl Legible for u8 {
    fn legible(&self) -> impl core::fmt::Display {
        struct Wrapper(u8);
        impl core::fmt::Display for Wrapper {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if 0x20 <= self.0 && self.0 <= 0x7e {
                    match self.0 {
                        b'\'' => write!(f, "'\\''"),
                        b'\\' => write!(f, "'\\\\'"),
                        c => write!(f, "'{}'", char::from(c)),
                    }
                } else {
                    write!(f, "'\\x{:02X}'", self.0)
                }
            }
        }
        Wrapper(*self)
    }

    fn colored(&self) -> impl core::fmt::Display {
        struct Wrapper(u8);
        impl core::fmt::Display for Wrapper {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                "'".bright_green().fmt(f)?;
                if 0x20 <= self.0 && self.0 <= 0x7e {
                    char::from(self.0).bold().bright_green().fmt(f)?;
                } else {
                    "\\x".bright_cyan().fmt(f)?;
                    write!(f, "{:02X}", self.0.bold().bright_cyan())?;
                }
                "'".bright_green().fmt(f)
            }
        }
        Wrapper(*self)
    }
}

impl Legible for [u8] {
    fn legible(&self) -> impl core::fmt::Display {
        struct Wrapper<'a>(&'a [u8]);
        impl core::fmt::Display for Wrapper<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_char('"')?;
                for byte in self.0 {
                    if 0x20 <= *byte && *byte <= 0x7e {
                        match *byte {
                            b'"' => write!(f, r#"\""#)?,
                            b'\\' => write!(f, r#"\\"#)?,
                            c => write!(f, "{}", char::from(c))?,
                        }
                    } else {
                        write!(f, "\\x{byte:02X}")?;
                    }
                }
                f.write_char('"')
            }
        }
        Wrapper(self)
    }

    fn colored(&self) -> impl core::fmt::Display {
        struct Wrapper<'a>(&'a [u8]);
        impl core::fmt::Display for Wrapper<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", '"'.bright_green())?;
                for byte in self.0 {
                    if 0x20 <= *byte && *byte <= 0x7e {
                        char::from(*byte).bold().bright_green().fmt(f)?;
                    } else {
                        "\\x".bright_cyan().fmt(f)?;
                        write!(f, "{:02X}", byte.bold().bright_cyan())?;
                    }
                }
                write!(f, "{}", '"'.bright_green())
            }
        }
        Wrapper(self)
    }
}

impl<const N: usize> Legible for [u8; N] {
    fn legible(&self) -> impl core::fmt::Display {
        self[..].legible()
    }

    fn colored(&self) -> impl core::fmt::Display {
        self[..].colored()
    }
}
