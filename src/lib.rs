#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub type FormatRuleFn = fn(&str, usize, usize) -> String;

/// Правило форматирования по умолчанию (вынесено в функцию для читаемости)
fn default_format_rule(val: &str, index: usize, len: usize) -> String {
    if len == 0 {
        return String::new();
    }

    let is_last = index == len - 1;

    if index == 0 {
        // Исправлен баг: если элемент один, он является и первым, и последним
        if is_last {
            format!("[{}]", val)
        } else {
            format!("[{}", val)
        }
    } else if is_last {
        format!(", {}]", val)
    } else {
        format!(", {}", val)
    }
}

pub const DEFAULT_FORMAT_RULE: FormatRuleFn = default_format_rule;

// ============================================================================
// НОВЫЕ ТРЕЙТЫ С ИМЕНОВАННЫМИ АРГУМЕНТАМИ
// ============================================================================

/// Правило форматирования БЕЗ состояния
pub trait FormatRuleNoStateLongLf<'a, F>
where
    F: Fn(&'a str, usize, usize) -> String + 'a,
{
    /// Форматирует один элемент коллекции.
    ///
    /// # Arguments
    /// * `value` - Строковое представление текущего элемента
    /// * `index` - Индекс текущего элемента (от 0)
    /// * `length` - Общее количество элементов в коллекции
    fn format(&'a self, value: &'a str, index: usize, length: usize) -> String;
}

impl<'a, F> FormatRuleNoStateLongLf<'a, F> for F
where
    F: Fn(&str, usize, usize) -> String + 'a,
{
    fn format(&'a self, value: &str, index: usize, length: usize) -> String {
        (self)(value, index, length)
    }
}

/// Правило форматирования БЕЗ состояния с изменяемым внутренним состоянием замыкания
pub trait FormatRuleMutNoState {
    /// Форматирует один элемент коллекции.
    ///
    /// # Arguments
    /// * `value` - Строковое представление текущего элемента
    /// * `index` - Индекс текущего элемента (от 0)
    /// * `length` - Общее количество элементов в коллекции
    fn format(&mut self, value: &str, index: usize, length: usize) -> String;
}

impl<F> FormatRuleMutNoState for F
where
    F: FnMut(&str, usize, usize) -> String,
{
    fn format(&mut self, value: &str, index: usize, length: usize) -> String {
        (self)(value, index, length)
    }
}

/// Правило форматирования с неизменяемым состоянием
pub trait FormatRule<S> {
    /// Форматирует один элемент коллекции.
    ///
    /// # Arguments
    /// * `state` - Ссылка на состояние (контекст), передаваемое между вызовами
    /// * `value` - Строковое представление текущего элемента
    /// * `index` - Индекс текущего элемента (от 0)
    /// * `length` - Общее количество элементов в коллекции
    fn format(&self, state: &S, value: &str, index: usize, length: usize) -> String;
}

impl<S, F> FormatRule<S> for F
where
    F: Fn(&S, &str, usize, usize) -> String,
{
    fn format(&self, state: &S, value: &str, index: usize, length: usize) -> String {
        (self)(state, value, index, length)
    }
}

/// Правило форматирования с изменяемым состоянием
pub trait FormatRuleMut<S> {
    /// Форматирует один элемент коллекции.
    ///
    /// # Arguments
    /// * `state` - Изменяемая ссылка на состояние (контекст), передаваемое между вызовами
    /// * `value` - Строковое представление текущего элемента
    /// * `index` - Индекс текущего элемента (от 0)
    /// * `length` - Общее количество элементов в коллекции
    fn format(&mut self, state: &mut S, value: &str, index: usize, length: usize) -> String;
}

impl<S, F> FormatRuleMut<S> for F
where
    F: FnMut(&mut S, &str, usize, usize) -> String,
{
    fn format(&mut self, state: &mut S, value: &str, index: usize, length: usize) -> String {
        (self)(state, value, index, length)
    }
}

// ============================================================================
// СТАРЫЕ ТРЕЙТЫ (ПОЛНАЯ ОБРАТНАЯ СОВМЕСТИМОСТЬ)
// ============================================================================

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
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&format_rule(&format!("{}", x), i, len));
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
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&format_rule(&format!("{}", x), i, len));
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
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&format_rule(&format!("{}", x), i, len));
        }
        string
    }
}

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

impl<I, T> IteratorString for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
{
    fn iter_string(self, format_rule: FormatRuleFn) -> String {
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

// Типажи с изменяемым состоянием (FnMut)
pub trait VecStringWithState<S, F>
where
    F: FnMut(&mut S, &str, usize, usize) -> String,
{
    fn vec_string_with_state(&self, initial_state: S, format_rule: F) -> String;
}

pub trait IteratorStringWithState<S, F>
where
    F: FnMut(&mut S, &str, usize, usize) -> String,
{
    fn iter_string_with_state(self, initial_state: S, format_rule: F) -> String;
}

impl<T, S, F> VecStringWithState<S, F> for Vec<T>
where
    T: core::fmt::Display,
    F: FnMut(&mut S, &str, usize, usize) -> String,
{
    fn vec_string_with_state(&self, mut initial_state: S, mut format_rule: F) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&format_rule(&mut initial_state, &s, i, len));
        }
        result
    }
}

impl<I, T, S, F> IteratorStringWithState<S, F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: FnMut(&mut S, &str, usize, usize) -> String,
{
    fn iter_string_with_state(self, mut initial_state: S, mut format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&mut initial_state, &s, i, len));
        }
        result
    }
}

// Типажи с неизменяемым состоянием (Fn)
pub trait VecStringWithStateFn<S, F>
where
    F: Fn(&S, &str, usize, usize) -> String,
{
    fn vec_string_with_state_fn(&self, state: &S, format_rule: F) -> String;
}

pub trait IteratorStringWithStateFn<S, F>
where
    F: Fn(&S, &str, usize, usize) -> String,
{
    fn iter_string_with_state_fn(self, state: &S, format_rule: F) -> String;
}

impl<T, S, F> VecStringWithStateFn<S, F> for Vec<T>
where
    T: core::fmt::Display,
    F: Fn(&S, &str, usize, usize) -> String,
{
    fn vec_string_with_state_fn(&self, state: &S, format_rule: F) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&format_rule(state, &s, i, len));
        }
        result
    }
}

impl<I, T, S, F> IteratorStringWithStateFn<S, F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: Fn(&S, &str, usize, usize) -> String,
{
    fn iter_string_with_state_fn(self, state: &S, format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(state, &s, i, len));
        }
        result
    }
}

// Типажи с неизменяемым состоянием (fn указатель)
pub trait VecStringWithStateFnPtr<S> {
    fn vec_string_with_state_fn_ptr(
        &self,
        state: &S,
        format_rule: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

pub trait IteratorStringWithStateFnPtr<S> {
    fn iter_string_with_state_fn_ptr(
        self,
        state: &S,
        format_rule: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

impl<T, S> VecStringWithStateFnPtr<S> for Vec<T>
where
    T: core::fmt::Display,
{
    fn vec_string_with_state_fn_ptr(
        &self,
        state: &S,
        format_rule: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&format_rule(state, &s, i, len));
        }
        result
    }
}

impl<I, T, S> IteratorStringWithStateFnPtr<S> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
{
    fn iter_string_with_state_fn_ptr(
        self,
        state: &S,
        format_rule: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(state, &s, i, len));
        }
        result
    }
}

// ============================================================================
// НОВЫЕ ТРЕЙТЫ С ИСПОЛЬЗОВАНИЕМ FormatRule (С ИМЕНОВАННЫМИ АРГУМЕНТАМИ)
// ============================================================================

/// Расширение для Vec с использованием FormatRuleNoState
pub trait VecStringRule<'a, R, F>
where
    R: FormatRuleNoStateLongLf<'a, F>,
    F: Fn(&'a str, usize, usize) -> String + 'a,
{
    fn vec_string_rule(&'a self, rule: R) -> String;
}

impl<'a, T, R, F> VecStringRule<'a, R, F> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleNoStateLongLf<'a, F>,
    F: Fn(&'a str, usize, usize) -> String + 'a,
{
    fn vec_string_rule(&'a self, rule: R) -> String {
        let mut string: String = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&rule.format(&format!("{}", x), i, len));
        }
        string
    }
}

/// Расширение для Vec с использованием FormatRuleMutNoState
pub trait VecStringMutRule<R>
where
    R: FormatRuleMutNoState,
{
    fn vec_string_mut_rule(&self, rule: R) -> String;
}

impl<T, R> VecStringMutRule<R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    fn vec_string_mut_rule(&self, mut rule: R) -> String {
        let mut string: String = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&rule.format(&format!("{}", x), i, len));
        }
        string
    }
}

/// Расширение для итераторов с использованием FormatRuleNoState
pub trait IteratorStringRule<'a, R, F>
where
    R: FormatRuleNoStateLongLf<'a, F>,
    F: Fn(&'a str, usize, usize) -> String + 'a,
{
    fn iter_string_rule(self, rule: R) -> String;
}

impl<'a, I, T, R, F> IteratorStringRule<'a, R, F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: Fn(&'a str, usize, usize) -> String + 'a,
    R: FormatRuleNoStateLongLf<'a, F> + 'a,
{
    fn iter_string_rule(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

/// Расширение для итераторов с использованием FormatRuleMutNoState
pub trait IteratorStringMutRule<R>
where
    R: FormatRuleMutNoState,
{
    fn iter_string_mut_rule(self, rule: R) -> String;
}

impl<I, T, R> IteratorStringMutRule<R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    fn iter_string_mut_rule(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

/// Расширение для Vec с использованием FormatRule (неизменяемое состояние)
pub trait VecStringWithStateRule<S, R>
where
    R: FormatRule<S>,
{
    fn vec_string_with_state_rule(&self, state: &S, rule: R) -> String;
}

impl<T, S, R> VecStringWithStateRule<S, R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    fn vec_string_with_state_rule(&self, state: &S, rule: R) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

/// Расширение для итераторов с использованием FormatRule (неизменяемое состояние)
pub trait IteratorStringWithStateRule<S, R>
where
    R: FormatRule<S>,
{
    fn iter_string_with_state_rule(self, state: &S, rule: R) -> String;
}

impl<I, T, S, R> IteratorStringWithStateRule<S, R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    fn iter_string_with_state_rule(self, state: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

/// Расширение для Vec с использованием FormatRuleMut (изменяемое состояние)
pub trait VecStringWithStateMutRule<S, R>
where
    R: FormatRuleMut<S>,
{
    fn vec_string_with_state_mut_rule(&self, initial_state: S, rule: R) -> String;
}

impl<T, S, R> VecStringWithStateMutRule<S, R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    fn vec_string_with_state_mut_rule(&self, mut initial_state: S, mut rule: R) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

/// Расширение для итераторов с использованием FormatRuleMut (изменяемое состояние)
pub trait IteratorStringWithStateMutRule<S, R>
where
    R: FormatRuleMut<S>,
{
    fn iter_string_with_state_mut_rule(self, initial_state: S, rule: R) -> String;
}

impl<I, T, S, R> IteratorStringWithStateMutRule<S, R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    fn iter_string_with_state_mut_rule(self, mut initial_state: S, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_vec_string_default() {
        assert_eq!(
            "[1, 2, 3]",
            VecString::vec_string(&vec![1, 2, 3], DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_vec_string_single_element() {
        assert_eq!(
            "[42]",
            VecString::vec_string(&vec![42], DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_vec_string_empty() {
        assert_eq!(
            "",
            VecString::vec_string(&Vec::<i32>::new(), DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iterator_string() {
        let numbers = vec![1, 2, 3];
        let s = IteratorString::iter_string(numbers.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE);
        assert_eq!("[10, 20, 30]", s);
    }

    #[test]
    fn test_iterator_string_empty() {
        let numbers: Vec<i32> = vec![];
        let s = IteratorString::iter_string(numbers.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE);
        assert_eq!("", s);
    }

    #[test]
    fn test_iterator_string_single() {
        let numbers = vec![42];
        let s = IteratorString::iter_string(numbers.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE);
        assert_eq!("[420]", s);
    }

    #[test]
    fn test_vec_string_fn() {
        let v = vec!["a", "bb", "ccc"];
        let res = VecStringFn::vec_string(&v, |val, idx, total| {
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("({})", val)
                } else {
                    format!("({}", val)
                }
            } else if is_last {
                format!(", {})", val)
            } else {
                format!(", {}", val)
            }
        });
        assert_eq!(res, "(a, bb, ccc)");
    }

    #[test]
    fn test_vec_string_fn_single() {
        let v = vec!["only"];
        let res = VecStringFn::vec_string(&v, |val, idx, total| {
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("({})", val)
                } else {
                    format!("({}", val)
                }
            } else if is_last {
                format!(", {})", val)
            } else {
                format!(", {}", val)
            }
        });
        assert_eq!(res, "(only)");
    }

    #[test]
    fn test_vec_string_fn_mut() {
        let v = vec!["x", "y", "z"];
        let mut counter = 0;
        let res = VecStringFnMut::vec_string(&v, |val, _idx, _total| {
            counter += 1;
            format!("{}{}", val, counter)
        });
        assert_eq!(res, "x1y2z3");
        assert_eq!(counter, 3);
    }

    #[test]
    fn test_vec_string_fn_mut_empty() {
        let v: Vec<&str> = vec![];
        let mut counter = 0;
        let res = VecStringFnMut::vec_string(&v, |val, _idx, _total| {
            counter += 1;
            format!("{}{}", val, counter)
        });
        assert_eq!(res, "");
        assert_eq!(counter, 0);
    }

    #[test]
    fn test_iterator_string_fn() {
        let v = vec![1, 2, 3];
        let res = IteratorStringFn::iter_string(v.iter(), |val, idx, total| {
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("{{{}}}", val)
                } else {
                    format!("{{{}", val)
                }
            } else if is_last {
                format!(", {}}}", val)
            } else {
                format!(", {}", val)
            }
        });
        assert_eq!(res, "{1, 2, 3}");
    }

    #[test]
    fn test_iterator_string_fn_mut() {
        let v = vec![10, 20, 30];
        let mut sum = 0;
        let res = IteratorStringFnMut::iter_string(v.iter(), |val, idx, total| {
            let num: i32 = val.parse().unwrap_or(0);
            sum += num;
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if is_last {
                format!("{} (sum={})", val, sum)
            } else {
                format!("{}, ", val)
            }
        });
        assert_eq!(res, "10, 20, 30 (sum=60)");
        assert_eq!(sum, 60);
    }

    #[test]
    fn test_stateful_vec() {
        let data = vec!["hello", "world", "rust"];
        let positions = [0usize, 1, 2].into_iter();

        let result = data.vec_string_with_state(positions, |pos, val, idx, total| {
            let start = pos.next().unwrap_or(0);
            let short = if val.len() > start {
                &val[start..]
            } else {
                val
            };
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("[{}]", short)
                } else {
                    format!("[{}", short)
                }
            } else if is_last {
                format!(", {}]", short)
            } else {
                format!(", {}", short)
            }
        });
        assert_eq!(result, "[hello, orld, st]");
    }

    #[test]
    fn test_iterator_string_with_state() {
        let data = vec![1, 2, 3].into_iter();
        #[allow(unused_mut)]
        let mut sum = 0;

        let result = data.iter_string_with_state(sum, |state, val, idx, total| {
            let num: i32 = val.parse().unwrap_or(0);
            *state += num;
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("(sum={}: {})", state, val)
                } else {
                    format!("(sum={}: {}", state, val)
                }
            } else if is_last {
                format!(", sum={}: {})", state, val)
            } else {
                format!(", sum={}: {}", state, val)
            }
        });
        assert_eq!(result, "(sum=1: 1, sum=3: 2, sum=6: 3)");
    }

    #[test]
    fn test_stateful_empty() {
        let data: Vec<&str> = vec![];
        let positions = [].into_iter();

        let result = data.vec_string_with_state(positions, |pos, val, idx, total| {
            let start = pos.next().unwrap_or(0);
            let short = if val.len() > start {
                &val[start..]
            } else {
                val
            };
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("[{}]", short)
                } else {
                    format!("[{}", short)
                }
            } else if is_last {
                format!(", {}]", short)
            } else {
                format!(", {}", short)
            }
        });
        assert_eq!(result, "");
    }

    // Тесты для неизменяемого состояния (Fn)
    #[test]
    fn test_vec_string_with_state_fn() {
        let data = vec!["hello", "world"];
        let prefix = ">>";

        let result = data.vec_string_with_state_fn(&prefix, |state, val, idx, total| {
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("[{}{}]", state, val)
                } else {
                    format!("[{}{}", state, val)
                }
            } else if is_last {
                format!(", {}{}]", state, val)
            } else {
                format!(", {}{}", state, val)
            }
        });
        assert_eq!(result, "[>>hello, >>world]");
    }

    #[test]
    fn test_iterator_string_with_state_fn() {
        let data = vec![1, 2, 3].into_iter();
        let multiplier = 10;

        let result = data.iter_string_with_state_fn(&multiplier, |state, val, idx, total| {
            let num: i32 = val.parse().unwrap_or(0);
            let formatted = format!("{}", num * state);
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("[{}]", formatted)
                } else {
                    format!("[{}", formatted)
                }
            } else if is_last {
                format!(", {}]", formatted)
            } else {
                format!(", {}", formatted)
            }
        });
        assert_eq!(result, "[10, 20, 30]");
    }

    #[test]
    fn test_vec_string_with_state_fn_empty() {
        let data: Vec<i32> = vec![];
        let prefix = ">>";

        let result = data.vec_string_with_state_fn(&prefix, |state, val, idx, total| {
            if total == 0 {
                return String::new();
            }
            let is_last = idx == total - 1;
            if idx == 0 {
                if is_last {
                    format!("[{}{}]", state, val)
                } else {
                    format!("[{}{}", state, val)
                }
            } else if is_last {
                format!(", {}{}]", state, val)
            } else {
                format!(", {}{}", state, val)
            }
        });
        assert_eq!(result, "");
    }

    // Тесты для неизменяемого состояния (fn указатель)
    fn format_with_prefix(prefix: &String, val: &str, idx: usize, total: usize) -> String {
        if total == 0 {
            return String::new();
        }
        let is_last = idx == total - 1;
        if idx == 0 {
            if is_last {
                format!("[{}{}]", prefix, val)
            } else {
                format!("[{}{}", prefix, val)
            }
        } else if is_last {
            format!(", {}{}]", prefix, val)
        } else {
            format!(", {}{}", prefix, val)
        }
    }

    #[test]
    fn test_vec_string_with_state_fn_ptr() {
        let data = vec!["a", "b", "c"];
        let prefix = ">>".to_string();

        let result = data.vec_string_with_state_fn_ptr(&prefix, format_with_prefix);
        assert_eq!(result, "[>>a, >>b, >>c]");
    }

    #[test]
    fn test_iterator_string_with_state_fn_ptr() {
        let data = vec!["x", "y"].into_iter();
        let prefix = "##".to_string();

        let result = data.iter_string_with_state_fn_ptr(&prefix, format_with_prefix);
        assert_eq!(result, "[##x, ##y]");
    }

    #[test]
    fn test_vec_string_with_state_fn_ptr_empty() {
        let data: Vec<&str> = vec![];
        let prefix = ">>".to_string();

        let result = data.vec_string_with_state_fn_ptr(&prefix, format_with_prefix);
        assert_eq!(result, "");
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ НОВЫХ ТРЕЙТОВ С ИМЕНОВАННЫМИ АРГУМЕНТАМИ
    // ========================================================================

    #[test]
    fn test_vec_string_rule() {
        let v = vec![1, 2, 3];
        let res = v.vec_string_rule(|value, index, length| {
            if length == 0 {
                return String::new();
            }
            let is_last = index == length - 1;
            if index == 0 {
                if is_last {
                    format!("<{}>", value)
                } else {
                    format!("<{}", value)
                }
            } else if is_last {
                format!(", {}>", value)
            } else {
                format!(", {}", value)
            }
        });
        assert_eq!(res, "<1, 2, 3>");
    }

    #[test]
    fn test_vec_string_mut_rule() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0;
        let res = v.vec_string_mut_rule(|value, _index, _length| {
            counter += 1;
            format!("[{}{}]", value, counter)
        });
        assert_eq!(res, "[a1][b2][c3]");
        assert_eq!(counter, 3);
    }

    #[test]
    fn test_iterator_string_rule() {
        let v = vec![10, 20, 30];
        let res = v.iter().iter_string_rule(|value, index, length| {
            if length == 0 {
                return String::new();
            }
            let is_last = index == length - 1;
            if index == 0 {
                if is_last {
                    format!("{{{}}}", value)
                } else {
                    format!("{{{}", value)
                }
            } else if is_last {
                format!(", {}}}", value)
            } else {
                format!(", {}", value)
            }
        });
        assert_eq!(res, "{10, 20, 30}");
    }

    #[test]
    fn test_iterator_string_mut_rule() {
        let v = vec![1, 2, 3];
        let mut sum = 0;
        let res = v.iter().iter_string_mut_rule(|value, index, length| {
            let num: i32 = value.parse().unwrap_or(0);
            sum += num;
            if length == 0 {
                return String::new();
            }
            let is_last = index == length - 1;
            if is_last {
                format!("{} (total={})", value, sum)
            } else {
                format!("{}, ", value)
            }
        });
        assert_eq!(res, "1, 2, 3 (total=6)");
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_vec_string_with_state_rule() {
        let data = vec!["hello", "world"];
        let prefix = ">>";

        let result = data.vec_string_with_state_rule(&prefix, |state, value, index, length| {
            if length == 0 {
                return String::new();
            }
            let is_last = index == length - 1;
            if index == 0 {
                if is_last {
                    format!("[{}{}]", state, value)
                } else {
                    format!("[{}{}", state, value)
                }
            } else if is_last {
                format!(", {}{}]", state, value)
            } else {
                format!(", {}{}", state, value)
            }
        });
        assert_eq!(result, "[>>hello, >>world]");
    }

    #[test]
    fn test_iterator_string_with_state_rule() {
        let data = vec![1, 2, 3].into_iter();
        let multiplier = 10;

        let result =
            data.iter_string_with_state_rule(&multiplier, |state, value, index, length| {
                let num: i32 = value.parse().unwrap_or(0);
                let formatted = format!("{}", num * state);
                if length == 0 {
                    return String::new();
                }
                let is_last = index == length - 1;
                if index == 0 {
                    if is_last {
                        format!("[{}]", formatted)
                    } else {
                        format!("[{}", formatted)
                    }
                } else if is_last {
                    format!(", {}]", formatted)
                } else {
                    format!(", {}", formatted)
                }
            });
        assert_eq!(result, "[10, 20, 30]");
    }

    #[test]
    fn test_vec_string_with_state_mut_rule() {
        let data = vec![1, 2, 3];
        let mut sum = 0;

        let result = data.vec_string_with_state_mut_rule(sum, |state, value, index, length| {
            let num: i32 = value.parse().unwrap_or(0);
            *state += num;
            if length == 0 {
                return String::new();
            }
            let is_last = index == length - 1;
            if index == 0 {
                if is_last {
                    format!("(sum={}: {})", state, value)
                } else {
                    format!("(sum={}: {}", state, value)
                }
            } else if is_last {
                format!(", sum={}: {})", state, value)
            } else {
                format!(", sum={}: {}", state, value)
            }
        });
        assert_eq!(result, "(sum=1: 1, sum=3: 2, sum=6: 3)");
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule() {
        let data = vec!["hello", "world", "rust"];
        let positions = [0usize, 1, 2].into_iter();

        let result =
            data.iter()
                .iter_string_with_state_mut_rule(positions, |pos, value, index, length| {
                    let start = pos.next().unwrap_or(0);
                    let short = if value.len() > start {
                        &value[start..]
                    } else {
                        value
                    };
                    if length == 0 {
                        return String::new();
                    }
                    let is_last = index == length - 1;
                    if index == 0 {
                        if is_last {
                            format!("[{}]", short)
                        } else {
                            format!("[{}", short)
                        }
                    } else if is_last {
                        format!(", {}]", short)
                    } else {
                        format!(", {}", short)
                    }
                });
        assert_eq!(result, "[hello, orld, st]");
    }
}
