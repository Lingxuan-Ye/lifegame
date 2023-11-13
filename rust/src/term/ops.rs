mod add {
    use super::super::{TermString, IntoTermString};
    use std::ops::{Add, AddAssign};

    impl AddAssign for TermString {
        fn add_assign(&mut self, rhs: Self) {
            self.data += &rhs.data;
            self.esc_len += rhs.esc_len;
        }
    }

    impl Add for TermString {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl AddAssign<String> for TermString {
        fn add_assign(&mut self, rhs: String) {
            self.data += &rhs;
        }
    }

    impl Add<String> for TermString {
        type Output = Self;
        fn add(mut self, rhs: String) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl Add<TermString> for String {
        type Output = TermString;
        fn add(self, rhs: TermString) -> Self::Output {
            self.into_tstr() + rhs
        }
    }

    impl AddAssign<&str> for TermString {
        fn add_assign(&mut self, rhs: &str) {
            self.data += rhs;
        }
    }

    impl Add<&str> for TermString {
        type Output = Self;
        fn add(mut self, rhs: &str) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl Add<TermString> for &str {
        type Output = TermString;
        fn add(self, rhs: TermString) -> Self::Output {
            self.into_tstr() + rhs
        }
    }
}

mod deref {
    use super::super::TermString;
    use std::ops::Deref;

    impl Deref for TermString {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
}
