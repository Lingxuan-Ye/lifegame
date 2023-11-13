use super::TermString;
use std::fmt;

impl fmt::Display for TermString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
