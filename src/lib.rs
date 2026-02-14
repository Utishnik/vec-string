#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub type FormatRuleFn = fn(&str, usize, usize) -> String;

pub const DEFAULT_FORMAT_RULE: FormatRuleFn = {
    fn format_def(val: &str, index: usize, len: usize) -> String {
        if index == 0 {
            format!("[{}", val)
        } else if index != len - 1 {
            format!(", {}", val)
        } else {
            format!(", {}]", val)
        }
    }
    format_def
};

pub trait VecString {
    fn vec_string(&self, format_rule: FormatRuleFn) -> String;
}

/// Get string of Vec<T> where T: Display
impl<T> VecString for Vec<T>
where
    T: core::fmt::Display,
{
    /// assert_eq!("1, 2, 3", vec![1, 2, 3].vec_string());
    fn vec_string(&self, format_rule: FormatRuleFn) -> String {
        let mut string: String = String::new();
        for x in self.iter().enumerate() {
            string.push_str(&format_rule(&format!("{}", x.1), x.0, self.len()));
        }
        string
    }
}

#[cfg(test)]
mod tests {
    use crate::VecString;
    use crate::DEFAULT_FORMAT_RULE;
    use alloc::vec;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test() {
        assert_eq!("[1, 2, 3]", vec![1, 2, 3].vec_string(DEFAULT_FORMAT_RULE));
    }
}
