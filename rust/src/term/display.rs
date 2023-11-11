use super::TermString;
use std::fmt::{Display, Formatter};

impl Display for TermString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
