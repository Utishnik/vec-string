#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================================
// ОБЩИЙ ТРЕЙТ ExtendedDisplay
// ============================================================================

/// Маркерный трейт, объединяющий все возможности форматирования.
///
/// Реализован для:
/// * `Vec<T>` (by value) и `&Vec<T>`;
/// * большинства стандартных итераторов (`Map`, `Filter`, `Take`, `Skip`,
///   `Rev`, `Cloned`, `Copied`, `Chain`, `Flatten`, `StepBy`, `Peekable`,
///   `Fuse`, `Once`, `Repeat`, `Empty`, `Range`, `RangeFrom`,
///   `RangeInclusive`, `slice::Iter`, `vec::IntoIter`, `array::IntoIter`,
///   `str::Chars`, `str::Lines`, `str::Bytes`, `str::SplitWhitespace` и др.);
/// * любого пользовательского типа через макрос `impl_extended_display!`.
///
/// Пример generic-границы:
/// ```rust,ignore
/// fn dump<T: ExtendedDisplay>(items: T) { ... }
/// ```
pub trait ExtendedDisplay {}

// --- Helper: все VecString* возможности с fn-правилами -----------------------
#[doc(hidden)]
pub trait VecStringAll:
    VecString
    + VecStringFn<fn(&str, usize, usize) -> String>
    + VecStringFnMut<fn(&str, usize, usize) -> String>
    + VecStringWithState<(), fn(&mut (), &str, usize, usize) -> String>
    + VecStringWithStateFn<(), fn(&(), &str, usize, usize) -> String>
    + VecStringWithStateFnPtr<()>
    + VecStringRuleOwned<fn(&str, usize, usize) -> String>
    + VecStringMutRuleOwned<fn(&str, usize, usize) -> String>
    + VecStringWithStateRuleOwned<(), fn(&(), &str, usize, usize) -> String>
    + VecStringWithStateMutRuleOwned<(), fn(&mut (), &str, usize, usize) -> String>
    + VecStringRuleRef<'static, fn(&str, usize, usize) -> String>
    + VecStringMutRuleRef<fn(&str, usize, usize) -> String>
    + VecStringWithStateRuleRef<(), fn(&(), &str, usize, usize) -> String>
    + VecStringWithStateMutRuleRef<(), fn(&mut (), &str, usize, usize) -> String>
{}

impl<T> VecStringAll for Vec<T>
where
    T: core::fmt::Display,
    Vec<T>: VecString,
    Vec<T>: VecStringFn<fn(&str, usize, usize) -> String>,
    Vec<T>: VecStringFnMut<fn(&str, usize, usize) -> String>,
    Vec<T>: VecStringWithState<(), fn(&mut (), &str, usize, usize) -> String>,
    Vec<T>: VecStringWithStateFn<(), fn(&(), &str, usize, usize) -> String>,
    Vec<T>: VecStringWithStateFnPtr<()>,
    Vec<T>: VecStringRuleOwned<fn(&str, usize, usize) -> String>,
    Vec<T>: VecStringMutRuleOwned<fn(&str, usize, usize) -> String>,
    Vec<T>: VecStringWithStateRuleOwned<(), fn(&(), &str, usize, usize) -> String>,
    Vec<T>: VecStringWithStateMutRuleOwned<(), fn(&mut (), &str, usize, usize) -> String>,
    Vec<T>: VecStringRuleRef<'static, fn(&str, usize, usize) -> String>,
    Vec<T>: VecStringMutRuleRef<fn(&str, usize, usize) -> String>,
    Vec<T>: VecStringWithStateRuleRef<(), fn(&(), &str, usize, usize) -> String>,
    Vec<T>: VecStringWithStateMutRuleRef<(), fn(&mut (), &str, usize, usize) -> String>,
{}

// --- Helper: все IteratorString* возможности с fn-правилами ------------------
#[doc(hidden)]
pub trait IteratorStringAll:
    IteratorString
    + IteratorStringFn<fn(&str, usize, usize) -> String>
    + IteratorStringFnMut<fn(&str, usize, usize) -> String>
    + IteratorStringWithState<(), fn(&mut (), &str, usize, usize) -> String>
    + IteratorStringWithStateFn<(), fn(&(), &str, usize, usize) -> String>
    + IteratorStringWithStateFnPtr<()>
    + IteratorStringRuleOwned<fn(&str, usize, usize) -> String>
    + IteratorStringMutRuleOwned<fn(&str, usize, usize) -> String>
    + IteratorStringWithStateRuleOwned<(), fn(&(), &str, usize, usize) -> String>
    + IteratorStringWithStateMutRuleOwned<(), fn(&mut (), &str, usize, usize) -> String>
    + IteratorStringRuleRef<'static, fn(&str, usize, usize) -> String>
    + IteratorStringMutRuleRef<fn(&str, usize, usize) -> String>
    + IteratorStringWithStateRuleRef<(), fn(&(), &str, usize, usize) -> String>
    + IteratorStringWithStateMutRuleRef<(), fn(&mut (), &str, usize, usize) -> String>
{}

impl<I> IteratorStringAll for I
where
    I: IteratorString
        + IteratorStringFn<fn(&str, usize, usize) -> String>
        + IteratorStringFnMut<fn(&str, usize, usize) -> String>
        + IteratorStringWithState<(), fn(&mut (), &str, usize, usize) -> String>
        + IteratorStringWithStateFn<(), fn(&(), &str, usize, usize) -> String>
        + IteratorStringWithStateFnPtr<()>
        + IteratorStringRuleOwned<fn(&str, usize, usize) -> String>
        + IteratorStringMutRuleOwned<fn(&str, usize, usize) -> String>
        + IteratorStringWithStateRuleOwned<(), fn(&(), &str, usize, usize) -> String>
        + IteratorStringWithStateMutRuleOwned<(), fn(&mut (), &str, usize, usize) -> String>
        + IteratorStringRuleRef<'static, fn(&str, usize, usize) -> String>
        + IteratorStringMutRuleRef<fn(&str, usize, usize) -> String>
        + IteratorStringWithStateRuleRef<(), fn(&(), &str, usize, usize) -> String>
        + IteratorStringWithStateMutRuleRef<(), fn(&mut (), &str, usize, usize) -> String>,
{}

// --- Vec<T> -----------------------------------------------------------------
impl<T> ExtendedDisplay for Vec<T> where Vec<T>: VecStringAll {}
impl<T> ExtendedDisplay for &Vec<T> where Vec<T>: VecStringAll {}

// --- core::iter адаптеры ----------------------------------------------------
impl<I, F> ExtendedDisplay for core::iter::Map<I, F>
where
    core::iter::Map<I, F>: IteratorStringAll,
{}
impl<I, P> ExtendedDisplay for core::iter::Filter<I, P>
where
    core::iter::Filter<I, P>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Take<I>
where
    core::iter::Take<I>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Skip<I>
where
    core::iter::Skip<I>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Fuse<I>
where
    core::iter::Fuse<I>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Peekable<I>
where
    I: Iterator<Item: IntoIterator>,
    core::iter::Peekable<I>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Rev<I>
where
    core::iter::Rev<I>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Cloned<I>
where
    core::iter::Cloned<I>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Copied<I>
where
    core::iter::Copied<I>: IteratorStringAll,
{}
impl<I, U> ExtendedDisplay for core::iter::Chain<I, U>
where
    core::iter::Chain<I, U>: IteratorStringAll,
{}
impl<I, F, U> ExtendedDisplay for core::iter::FlatMap<I, F,U>
where
    F: Iterator<Item: IntoIterator>,
    core::iter::FlatMap<I, F,U>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Flatten<I>
where
    I: Iterator<Item: IntoIterator>,
    core::iter::Flatten<I>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::StepBy<I>
where
    core::iter::StepBy<I>: IteratorStringAll,
{}
impl<I, P> ExtendedDisplay for core::iter::FilterMap<I, P>
where
    core::iter::FilterMap<I, P>: IteratorStringAll,
{}
impl<I, F> ExtendedDisplay for core::iter::Inspect<I, F>
where
    core::iter::Inspect<I, F>: IteratorStringAll,
{}
impl<I, St, F> ExtendedDisplay for core::iter::Scan<I, St, F>
where
    core::iter::Scan<I, St, F>: IteratorStringAll,
{}
impl<I, P> ExtendedDisplay for core::iter::SkipWhile<I, P>
where
    core::iter::SkipWhile<I, P>: IteratorStringAll,
{}
impl<I, P> ExtendedDisplay for core::iter::TakeWhile<I, P>
where
    core::iter::TakeWhile<I, P>: IteratorStringAll,
{}
impl<I, F> ExtendedDisplay for core::iter::MapWhile<I, F>
where
    core::iter::MapWhile<I, F>: IteratorStringAll,
{}
impl<I> ExtendedDisplay for core::iter::Cycle<I>
where
    core::iter::Cycle<I>: IteratorStringAll,
{}
// --- core::iter источники ---------------------------------------------------
impl<T> ExtendedDisplay for core::iter::Once<T>
where
    core::iter::Once<T>: IteratorStringAll,
{}
impl<T: Clone> ExtendedDisplay for core::iter::Repeat<T>
where
    core::iter::Repeat<T>: IteratorStringAll,
{}
impl<F> ExtendedDisplay for core::iter::RepeatWith<F>
where
    core::iter::RepeatWith<F>: IteratorStringAll,
{}
impl<T> ExtendedDisplay for core::iter::Empty<T>
where
    core::iter::Empty<T>: IteratorStringAll,
{}
impl<F> ExtendedDisplay for core::iter::FromFn<F>
where
    core::iter::FromFn<F>: IteratorStringAll,
{}
impl<T, F> ExtendedDisplay for core::iter::Successors<T, F>
where
    core::iter::Successors<T, F>: IteratorStringAll,
{}

// --- Ranges -----------------------------------------------------------------
impl<T> ExtendedDisplay for core::ops::Range<T>
where
    core::ops::Range<T>: IteratorStringAll,
{}
impl<T> ExtendedDisplay for core::ops::RangeFrom<T>
where
    core::ops::RangeFrom<T>: IteratorStringAll,
{}
impl<T> ExtendedDisplay for core::ops::RangeInclusive<T>
where
    core::ops::RangeInclusive<T>: IteratorStringAll,
{}

// --- slice / vec / array / option / result ----------------------------------
impl<'a, T> ExtendedDisplay for core::slice::Iter<'a, T>
where
    core::slice::Iter<'a, T>: IteratorStringAll,
{}
impl<T> ExtendedDisplay for alloc::vec::IntoIter<T>
where
    alloc::vec::IntoIter<T>: IteratorStringAll,
{}
impl<T, const N: usize> ExtendedDisplay for core::array::IntoIter<T, N>
where
    core::array::IntoIter<T, N>: IteratorStringAll,
{}
impl<'a, T> ExtendedDisplay for core::option::Iter<'a, T>
where
    core::option::Iter<'a, T>: IteratorStringAll,
{}
impl<'a, T> ExtendedDisplay for core::result::Iter<'a, T>
where
    core::result::Iter<'a, T>: IteratorStringAll,
{}

// --- str итераторы ----------------------------------------------------------
impl<'a> ExtendedDisplay for core::str::Chars<'a>
where
    core::str::Chars<'a>: IteratorStringAll,
{}
impl<'a> ExtendedDisplay for core::str::Lines<'a>
where
    core::str::Lines<'a>: IteratorStringAll,
{}
impl<'a> ExtendedDisplay for core::str::Bytes<'a>
where
    core::str::Bytes<'a>: IteratorStringAll,
{}
impl<'a> ExtendedDisplay for core::str::SplitWhitespace<'a>
where
    core::str::SplitWhitespace<'a>: IteratorStringAll,
{}

// --- Макрос для пользовательских итераторов ---------------------------------
#[macro_export]
macro_rules! impl_extended_display {
    ($($ty:ty),* $(,)?) => {
        $(impl $crate::ExtendedDisplay for $ty {})*
    };
}

pub type FormatRuleFn = fn(&str, usize, usize) -> String;

fn default_format_rule(val: &str, index: usize, len: usize) -> String {
    if len == 0 {
        return String::new();
    }
    let is_last = index == len - 1;
    if index == 0 {
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
// ТРЕЙТЫ ПРАВИЛ ФОРМАТИРОВАНИЯ
// ============================================================================

pub trait FormatRuleNoState<'a> {
    fn format(&'a self, value: &str, index: usize, length: usize) -> String;
}

impl<'a, F> FormatRuleNoState<'a> for F
where
    F: Fn(&str, usize, usize) -> String,
{
    fn format(&'a self, value: &str, index: usize, length: usize) -> String {
        (self)(value, index, length)
    }
}

pub trait FormatRuleNoStateOwned {
    fn format(self, value: &str, index: usize, length: usize) -> String;
}

impl<F> FormatRuleNoStateOwned for F
where
    F: Fn(&str, usize, usize) -> String,
{
    fn format(self, value: &str, index: usize, length: usize) -> String {
        (self)(value, index, length)
    }
}

pub trait FormatRuleMutNoState {
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

pub trait FormatRule<S> {
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

pub trait FormatRuleMut<S> {
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
// СТАРЫЕ ТРЕЙТЫ (ОБРАТНАЯ СОВМЕСТИМОСТЬ)
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
        let mut string = String::new();
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
        let mut string = String::new();
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
        let mut string = String::new();
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
    #[inline(always)]
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
    #[inline(always)]
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
// НОВЫЕ ТРЕЙТЫ: ВЕРСИИ С ВЛАДЕНИЕМ (rule: R)
// ============================================================================

pub trait VecStringRuleOwned<R>
where
    R: FormatRuleNoStateOwned,
{
    fn vec_string_rule_owned(self, rule: R) -> String;
}

impl<T, R> VecStringRuleOwned<R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleNoStateOwned + Clone,
{
    #[inline(always)]
    fn vec_string_rule_owned(self, rule: R) -> String {
        let mut string = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&rule.clone().format(&format!("{}", x), i, len).to_string());
        }
        string
    }
}

pub trait VecStringMutRuleOwned<R>
where
    R: FormatRuleMutNoState,
{
    fn vec_string_mut_rule_owned(&self, rule: R) -> String;
}

impl<T, R> VecStringMutRuleOwned<R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    #[inline(always)]
    fn vec_string_mut_rule_owned(&self, mut rule: R) -> String {
        let mut string = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&rule.format(&format!("{}", x), i, len));
        }
        string
    }
}

pub trait IteratorStringRuleOwned<R>
where
    R: FormatRuleNoStateOwned,
{
    fn iter_string_rule_owned(self, rule: R) -> String;
}

impl<I, T, R> IteratorStringRuleOwned<R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleNoStateOwned + Clone,
{
    #[inline(always)]
    fn iter_string_rule_owned(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.clone().format(&s, i, len));
        }
        result
    }
}

pub trait IteratorStringMutRuleOwned<R>
where
    R: FormatRuleMutNoState,
{
    fn iter_string_mut_rule_owned(self, rule: R) -> String;
}

impl<I, T, R> IteratorStringMutRuleOwned<R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    #[inline(always)]
    fn iter_string_mut_rule_owned(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

pub trait VecStringWithStateRuleOwned<S, R>
where
    R: FormatRule<S>,
{
    fn vec_string_with_state_rule_owned(&self, state: &S, rule: R) -> String;
}

impl<T, S, R> VecStringWithStateRuleOwned<S, R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    #[inline(always)]
    fn vec_string_with_state_rule_owned(&self, state: &S, rule: R) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

pub trait IteratorStringWithStateRuleOwned<S, R>
where
    R: FormatRule<S>,
{
    fn iter_string_with_state_rule_owned(self, state: &S, rule: R) -> String;
}

impl<I, T, S, R> IteratorStringWithStateRuleOwned<S, R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    #[inline(always)]
    fn iter_string_with_state_rule_owned(self, state: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

pub trait VecStringWithStateMutRuleOwned<S, R>
where
    R: FormatRuleMut<S>,
{
    fn vec_string_with_state_mut_rule_owned(&self, initial_state: S, rule: R) -> String;
}

impl<T, S, R> VecStringWithStateMutRuleOwned<S, R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    #[inline(always)]
    fn vec_string_with_state_mut_rule_owned(&self, mut initial_state: S, mut rule: R) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

pub trait IteratorStringWithStateMutRuleOwned<S, R>
where
    R: FormatRuleMut<S>,
{
    fn iter_string_with_state_mut_rule_owned(self, initial_state: S, rule: R) -> String;
}

impl<I, T, S, R> IteratorStringWithStateMutRuleOwned<S, R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    #[inline(always)]
    fn iter_string_with_state_mut_rule_owned(self, mut initial_state: S, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

// ============================================================================
// НОВЫЕ ТРЕЙТЫ: ВЕРСИИ ПО ССЫЛКЕ (rule: &R)
// ============================================================================

pub trait VecStringRuleRef<'a, R>
where
    R: FormatRuleNoState<'a>,
{
    fn vec_string_rule_ref(&self, rule: &'a R) -> String;
}

impl<'a, T, R> VecStringRuleRef<'a, R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleNoState<'a>,
{
    #[inline(always)]
    fn vec_string_rule_ref(&self, rule: &'a R) -> String {
        let mut string = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&rule.format(&format!("{}", x), i, len));
        }
        string
    }
}

pub trait VecStringMutRuleRef<R>
where
    R: FormatRuleMutNoState,
{
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String;
}

impl<T, R> VecStringMutRuleRef<R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    #[inline(always)]
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String {
        let mut string = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            string.push_str(&rule.format(&format!("{}", x), i, len));
        }
        string
    }
}

pub trait IteratorStringRuleRef<'a, R>
where
    R: FormatRuleNoState<'a>,
{
    fn iter_string_rule_ref(self, rule: &'a R) -> String;
}

impl<'a, I, T, R> IteratorStringRuleRef<'a, R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleNoState<'a>,
{
    #[inline(always)]
    fn iter_string_rule_ref(self, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

pub trait IteratorStringMutRuleRef<R>
where
    R: FormatRuleMutNoState,
{
    fn iter_string_mut_rule_ref(self, rule: &mut R) -> String;
}

impl<I, T, R> IteratorStringMutRuleRef<R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    #[inline(always)]
    fn iter_string_mut_rule_ref(self, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

pub trait VecStringWithStateRuleRef<S, R>
where
    R: FormatRule<S>,
{
    fn vec_string_with_state_rule_ref(&self, state: &S, rule: &R) -> String;
}

impl<T, S, R> VecStringWithStateRuleRef<S, R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    #[inline(always)]
    fn vec_string_with_state_rule_ref(&self, state: &S, rule: &R) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

pub trait IteratorStringWithStateRuleRef<S, R>
where
    R: FormatRule<S>,
{
    fn iter_string_with_state_rule_ref(self, state: &S, rule: &R) -> String;
}

impl<I, T, S, R> IteratorStringWithStateRuleRef<S, R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    #[inline(always)]
    fn iter_string_with_state_rule_ref(self, state: &S, rule: &R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

pub trait VecStringWithStateMutRuleRef<S, R>
where
    R: FormatRuleMut<S>,
{
    fn vec_string_with_state_mut_rule_ref(&self, initial_state: S, rule: &mut R) -> String;
}

impl<T, S, R> VecStringWithStateMutRuleRef<S, R> for Vec<T>
where
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    #[inline(always)]
    fn vec_string_with_state_mut_rule_ref(&self, mut initial_state: S, rule: &mut R) -> String {
        let mut result = String::new();
        let len = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

pub trait IteratorStringWithStateMutRuleRef<S, R>
where
    R: FormatRuleMut<S>,
{
    fn iter_string_with_state_mut_rule_ref(self, initial_state: S, rule: &mut R) -> String;
}

impl<I, T, S, R> IteratorStringWithStateMutRuleRef<S, R> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    #[inline(always)]
    fn iter_string_with_state_mut_rule_ref(self, mut initial_state: S, rule: &mut R) -> String {
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
    // ТЕСТЫ ДЛЯ НОВЫХ ТРЕЙТОВ: ВЕРСИИ С ВЛАДЕНИЕМ
    // ========================================================================

    #[test]
    fn test_vec_string_rule_owned() {
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
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
        };
        let res = v.vec_string_rule_owned(fmt);
        assert_eq!(res, "<1, 2, 3>");
    }

    #[test]
    fn test_vec_string_mut_rule_owned() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0;
        let fmt = |value: &str, _index: usize, _length: usize| {
            counter += 1;
            format!("[{}{}]", value, counter)
        };
        let res = v.vec_string_mut_rule_owned(fmt);
        assert_eq!(res, "[a1][b2][c3]");
        assert_eq!(counter, 3);
    }

    #[test]
    fn test_iterator_string_rule_owned() {
        let v = vec![10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
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
        };
        let res = v.iter().iter_string_rule_owned(fmt);
        assert_eq!(res, "{10, 20, 30}");
    }

    #[test]
    fn test_iterator_string_mut_rule_owned() {
        let v = vec![1, 2, 3];
        let mut sum = 0;
        let fmt = |value: &str, index: usize, length: usize| {
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
        };
        let res = v.iter().iter_string_mut_rule_owned(fmt);
        assert_eq!(res, "1, 2, 3 (total=6)");
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_vec_string_with_state_rule_owned() {
        let data = vec!["hello", "world"];
        let prefix = ">>";
        let fmt = |state: &&str, value: &str, index: usize, length: usize| {
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
        };
        let result = data.vec_string_with_state_rule_owned(&prefix, fmt);
        assert_eq!(result, "[>>hello, >>world]");
    }

    #[test]
    fn test_iterator_string_with_state_rule_owned() {
        let data = vec![1, 2, 3].into_iter();
        let multiplier = 10;
        let fmt = |state: &i32, value: &str, index: usize, length: usize| {
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
        };
        let result = data.iter_string_with_state_rule_owned(&multiplier, fmt);
        assert_eq!(result, "[10, 20, 30]");
    }

    #[test]
    fn test_vec_string_with_state_mut_rule_owned() {
        let data = vec![1, 2, 3];
        #[allow(unused_mut)]
        let mut sum = 0;
        let fmt = |state: &mut i32, value: &str, index: usize, length: usize| {
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
        };
        let result = data.vec_string_with_state_mut_rule_owned(sum, fmt);
        assert_eq!(result, "(sum=1: 1, sum=3: 2, sum=6: 3)");
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule_owned() {
        let data: Vec<&str> = vec!["hello", "world", "rust"];
        let positions: std::array::IntoIter<usize, 3> = [0usize, 1, 2].into_iter();
        let fmt =
            |pos: &mut std::array::IntoIter<usize, 3>, value: &str, index: usize, length: usize| {
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
            };
        let result = data
            .iter()
            .iter_string_with_state_mut_rule_owned(positions, fmt);
        assert_eq!(result, "[hello, orld, st]");
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ НОВЫХ ТРЕЙТОВ: ВЕРСИИ ПО ССЫЛКЕ
    // ========================================================================

    #[test]
    fn test_vec_string_rule_ref() {
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
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
        };
        let res = v.vec_string_rule_ref(&fmt);
        assert_eq!(res, "<1, 2, 3>");
    }

    #[test]
    fn test_vec_string_mut_rule_ref() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0;
        let mut fmt = |value: &str, _index: usize, _length: usize| {
            counter += 1;
            format!("[{}{}]", value, counter)
        };
        let res = v.vec_string_mut_rule_ref(&mut fmt);
        assert_eq!(res, "[a1][b2][c3]");
        assert_eq!(counter, 3);
    }

    #[test]
    fn test_iterator_string_rule_ref() {
        let v = vec![10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
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
        };
        let res = v.iter().iter_string_rule_ref(&fmt);
        assert_eq!(res, "{10, 20, 30}");
    }

    #[test]
    fn test_iterator_string_mut_rule_ref() {
        let v = vec![1, 2, 3];
        let mut sum = 0;
        let mut fmt = |value: &str, index: usize, length: usize| {
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
        };
        let res = v.iter().iter_string_mut_rule_ref(&mut fmt);
        assert_eq!(res, "1, 2, 3 (total=6)");
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_vec_string_with_state_rule_ref() {
        let data = vec!["hello", "world"];
        let prefix = ">>";
        let fmt = |state: &&str, value: &str, index: usize, length: usize| -> String {
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
        };
        let result = data.vec_string_with_state_rule_ref(&prefix, &fmt);
        assert_eq!(result, "[>>hello, >>world]");
    }

    #[test]
    fn test_iterator_string_with_state_rule_ref() {
        let data = vec![1, 2, 3].into_iter();
        let multiplier = 10;
        let fmt = |state: &i32, value: &str, index: usize, length: usize| -> String {
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
        };
        let result = data.iter_string_with_state_rule_ref(&multiplier, &fmt);
        assert_eq!(result, "[10, 20, 30]");
    }

    #[test]
    fn test_vec_string_with_state_mut_rule_ref() {
        let data = vec![1, 2, 3];
        #[allow(unused_mut)]
        let mut sum = 0;
        let mut fmt = |state: &mut i32, value: &str, index: usize, length: usize| {
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
        };
        let result = data.vec_string_with_state_mut_rule_ref(sum, &mut fmt);
        assert_eq!(result, "(sum=1: 1, sum=3: 2, sum=6: 3)");
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule_ref() {
        let data: Vec<&str> = vec!["hello", "world", "rust"];
        #[allow(unused_mut)]
        let mut positions = [0usize, 1, 2].into_iter();
        let mut fmt = |pos: &mut std::array::IntoIter<usize, 3>, value: &str, index, length| {
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
        };
        let result = data
            .iter()
            .iter_string_with_state_mut_rule_ref(positions, &mut fmt);
        assert_eq!(result, "[hello, orld, st]");
    }

    // ========================================================================
    // ТЕСТЫ ExtendedDisplay
    // ========================================================================

    #[test]
    fn test_extended_display_vec() {
        let v = vec![1, 2, 3];
        fn takes_extended<T: ExtendedDisplay>(_x: T) {}
        takes_extended(v);
    }

    #[test]
    fn test_extended_display_iter() {
        let v = vec![1, 2, 3];
        fn takes_extended<T: ExtendedDisplay>(_x: T) {}
        takes_extended(v.iter());
        takes_extended(v.iter().map(|x| x * 2));
        takes_extended(v.into_iter());
    }
}