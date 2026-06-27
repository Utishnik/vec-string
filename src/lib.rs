#![cfg_attr(not(feature = "std"), no_std)]

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

// ----------------- типажи для Vec<T> без итераторов-----------------
pub trait VecString {
    fn vec_string(&self, format_rule: FormatRuleFn) -> String;
}

pub trait VecStringFn<F>
where
    F: Fn(&str, usize, usize) -> String,
{
    fn vec_string(&self, format_rule: F) -> String;
}

pub trait VecStringFnMut<F>
where
    F: FnMut(&str, usize, usize) -> String,
{
    fn vec_string(&self, format_rule: F) -> String;
}

impl<T> VecString for Vec<T>
where
    T: core::fmt::Display,
{
    fn vec_string(&self, format_rule: FormatRuleFn) -> String {
        let mut string: String = String::new();
        for x in self.iter().enumerate() {
            string.push_str(&format_rule(&format!("{}", x.1), x.0, self.len()));
        }
        string
    }
}

impl<T, F> VecStringFn<F> for Vec<T>
where
    T: core::fmt::Display,
    F: Fn(&str, usize, usize) -> String,
{
    fn vec_string(&self, format_rule: F) -> String {
        let mut string: String = String::new();
        for x in self.iter().enumerate() {
            string.push_str(&format_rule(&format!("{}", x.1), x.0, self.len()));
        }
        string
    }
}

impl<T, F> VecStringFnMut<F> for Vec<T>
where
    T: core::fmt::Display,
    F: FnMut(&str, usize, usize) -> String,
{
    fn vec_string(&self, mut format_rule: F) -> String {
        let mut string: String = String::new();
        for x in self.iter().enumerate() {
            string.push_str(&format_rule(&format!("{}", x.1), x.0, self.len()));
        }
        string
    }
}

// ----------------- Новые типажи для любых итераторов -----------------
pub trait IteratorString {
    fn iter_string(self, format_rule: FormatRuleFn) -> String;
}

pub trait IteratorStringFn<F>
where
    F: Fn(&str, usize, usize) -> String,
{
    fn iter_string(self, format_rule: F) -> String;
}

pub trait IteratorStringFnMut<F>
where
    F: FnMut(&str, usize, usize) -> String,
{
    fn iter_string(self, format_rule: F) -> String;
}

// Реализация для любого итератора над элементами, реализующими Display
impl<I, T> IteratorString for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
{
    fn iter_string(self, format_rule: FormatRuleFn) -> String {
        // Собираем строковые представления, чтобы знать общую длину
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&s, i, len));
        }
        result
    }
}

impl<I, T, F> IteratorStringFn<F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: Fn(&str, usize, usize) -> String,
{
    fn iter_string(self, format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&s, i, len));
        }
        result
    }
}

impl<I, T, F> IteratorStringFnMut<F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: FnMut(&str, usize, usize) -> String,
{
    fn iter_string(self, mut format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&s, i, len));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::IteratorString; // новый импорт
    use crate::VecString;
    use crate::DEFAULT_FORMAT_RULE;
    use alloc::vec;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_vec_string() {
        // старый способ через Vec
        assert_eq!("[1, 2, 3]", vec![1, 2, 3].vec_string(DEFAULT_FORMAT_RULE));
    }

    #[test]
    fn test_iterator_string() {
        // новый способ: через итератор, можно добавить map, slice и т.п.
        let numbers = vec![1, 2, 3];

        // просто итератор
        assert_eq!(
            "[1, 2, 3]",
            numbers
                .iter()
                .map(|x| x.to_string())
                .iter_string(DEFAULT_FORMAT_RULE)
        );

        // итератор с map (умножаем на 10)
        assert_eq!(
            "[10, 20, 30]",
            numbers
                .iter()
                .map(|x| x * 10)
                .iter_string(DEFAULT_FORMAT_RULE)
        );

        // срез через итератор
        assert_eq!(
            "[2, 3]",
            numbers[1..].iter().iter_string(DEFAULT_FORMAT_RULE)
        );
    }
}
