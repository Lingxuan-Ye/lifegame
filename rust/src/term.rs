mod add;
mod utils;

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ESCSEQ: Lazy<HashMap<String, HashMap<String, String>>> =
    Lazy::new(|| serde_json::from_str(include_str!("../resources/escseq.json")).unwrap());

pub struct TermString {
    data: String,
    esc_len: usize,
}

impl TermString {
    pub fn wrap<T: ToString>(seq: T) -> Self {
        TermString {
            data: seq.to_string(),
            esc_len: 0,
        }
    }
    pub fn from_escseq(escseq: &str) -> Self {
        TermString {
            data: escseq.to_string(),
            esc_len: escseq.len(),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len() - self.esc_len
    }
    pub fn center(mut self, mut width: usize) -> Self {
        width += self.esc_len;
        self.data = format!("{:^width$}", self.data);
        self
    }
    pub fn ljust(mut self, mut width: usize) -> Self {
        width += self.esc_len;
        self.data = format!("{:<width$}", self.data);
        self
    }
    pub fn rjust(mut self, mut width: usize) -> Self {
        width += self.esc_len;
        self.data = format!("{:>width$}", self.data);
        self
    }
}
