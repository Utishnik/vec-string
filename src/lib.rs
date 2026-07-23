#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    any(feature = "dyn_async", feature = "impl_async"),
    feature(async_fn_traits, unboxed_closures)
)]
#![cfg_attr(feature = "impl_async", feature(type_alias_impl_trait))]
#![cfg_attr(feature = "rayon", feature(allocator_api))]
#![feature(loop_hints)]
#![feature(auto_traits)]
#![feature(negative_impls)]

extern crate alloc;

#[cfg(any(feature = "dyn_async", feature = "impl_async"))]
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use core::pin::pin;

// ============================================================================
// РАЗДЕЛЕНИЕ Vec И Iterator ЧЕРЕЗ auto_trait + negative_impls
// ============================================================================

/// Auto-trait: реализуется всеми типами *кроме* `Vec<T>`.
/// Благодаря `negative_impls` мы можем явно отозвать реализацию у `Vec<_>`,
/// что разрешает делать два blanket impl `ExtendedDisplay` без конфликта.
auto trait NotVec {}

/// Явно запрещаем `Vec<T>` реализовывать `NotVec`.
impl<T, A> !NotVec for Vec<T, A> {}

// ============================================================================
// ОБЩИЙ ТРЕЙТ ExtendedDisplay
// ============================================================================

/// Маркерный трейт, объединяющий **все** возможности форматирования.
///
/// Реализуется автоматически для:
/// - `Vec<T>` (где `T: Display`) — через набор `VecString*`;
/// - любого `I: Iterator<Item = T>` (кроме `Vec`) — через набор `IteratorString*`;
/// - любого `I: ParallelIterator<Item = T>` (при фиче `rayon`) — через `ParIteratorString*`.
///
/// Пример использования в generic-границах:
/// ```rust,ignore
/// fn dump<T: ExtendedDisplay>(items: T) { ... }
/// ```
pub trait ExtendedDisplay {}

// --- Blanket impl для Vec<T> ------------------------------------------------
impl<T> ExtendedDisplay for Vec<T>
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
{
}

// --- Blanket impl для Iterator (все, кто NotVec) ----------------------------
impl<I, T> ExtendedDisplay for I
where
    I: Iterator<Item = T> + NotVec,
    T: core::fmt::Display,
    I: IteratorString,
    I: IteratorStringFn<fn(&str, usize, usize) -> String>,
    I: IteratorStringFnMut<fn(&str, usize, usize) -> String>,
    I: IteratorStringWithState<(), fn(&mut (), &str, usize, usize) -> String>,
    I: IteratorStringWithStateFn<(), fn(&(), &str, usize, usize) -> String>,
    I: IteratorStringWithStateFnPtr<()>,
    I: IteratorStringRuleOwned<fn(&str, usize, usize) -> String>,
    I: IteratorStringMutRuleOwned<fn(&str, usize, usize) -> String>,
    I: IteratorStringWithStateRuleOwned<(), fn(&(), &str, usize, usize) -> String>,
    I: IteratorStringWithStateMutRuleOwned<(), fn(&mut (), &str, usize, usize) -> String>,
    I: IteratorStringRuleRef<'static, fn(&str, usize, usize) -> String>,
    I: IteratorStringMutRuleRef<fn(&str, usize, usize) -> String>,
    I: IteratorStringWithStateRuleRef<(), fn(&(), &str, usize, usize) -> String>,
    I: IteratorStringWithStateMutRuleRef<(), fn(&mut (), &str, usize, usize) -> String>,
{
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
// SYNC: ТРЕЙТЫ ПРАВИЛ ФОРМАТИРОВАНИЯ
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
// SYNC: VecString* и IteratorString*
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(state, &s, i, len));
        }
        result
    }
}

// ============================================================================
// SYNC: НОВЫЕ ТРЕЙТЫ — ВЕРСИИ С ВЛАДЕНИЕМ (rule: R)
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
        #[unroll(full)]
        for (i, x) in self.iter().enumerate() {
            string.push_str(&rule.clone().format(&format!("{}", x), i, len));
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

// ============================================================================
// SYNC: НОВЫЕ ТРЕЙТЫ — ВЕРСИИ ПО ССЫЛКЕ (rule: &R)
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
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
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

// ============================================================================
// #[cfg(feature = "rayon")] — SYNC ParIterator*
// ============================================================================

#[cfg(feature = "rayon")]
pub trait ParIteratorString {
    fn par_iter_string(self, format_rule: FormatRuleFn) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T> ParIteratorString for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
{
    fn par_iter_string(self, format_rule: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringFn<F>
where
    F: Fn(&str, usize, usize) -> String,
{
    fn par_iter_string(self, format_rule: F) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, F> ParIteratorStringFn<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: Fn(&str, usize, usize) -> String,
{
    fn par_iter_string(self, format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringFnMut<F>
where
    F: FnMut(&str, usize, usize) -> String,
{
    fn par_iter_string(self, format_rule: F) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, F> ParIteratorStringFnMut<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: FnMut(&str, usize, usize) -> String,
{
    fn par_iter_string(self, mut format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithState<S, F>
where
    F: FnMut(&mut S, &str, usize, usize) -> String,
{
    fn par_iter_string_with_state(self, initial_state: S, format_rule: F) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, S, F> ParIteratorStringWithState<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: FnMut(&mut S, &str, usize, usize) -> String,
{
    fn par_iter_string_with_state(self, mut initial_state: S, mut format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(&mut initial_state, &s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateFn<S, F>
where
    F: Fn(&S, &str, usize, usize) -> String,
{
    fn par_iter_string_with_state_fn(self, state: &S, format_rule: F) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, S, F> ParIteratorStringWithStateFn<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: Fn(&S, &str, usize, usize) -> String,
{
    fn par_iter_string_with_state_fn(self, state: &S, format_rule: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(state, &s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateFnPtr<S> {
    fn par_iter_string_with_state_fn_ptr(
        self,
        state: &S,
        format_rule: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, S> ParIteratorStringWithStateFnPtr<S> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
{
    #[inline(always)]
    fn par_iter_string_with_state_fn_ptr(
        self,
        state: &S,
        format_rule: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&format_rule(state, &s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringRuleOwned<R>
where
    R: FormatRuleNoStateOwned,
{
    fn par_iter_string_rule_owned(self, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, R> ParIteratorStringRuleOwned<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleNoStateOwned + Clone,
{
    #[inline(always)]
    fn par_iter_string_rule_owned(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.clone().format(&s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringMutRuleOwned<R>
where
    R: FormatRuleMutNoState,
{
    fn par_iter_string_mut_rule_owned(self, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, R> ParIteratorStringMutRuleOwned<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    #[inline(always)]
    fn par_iter_string_mut_rule_owned(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateRuleOwned<S, R>
where
    R: FormatRule<S>,
{
    fn par_iter_string_with_state_rule_owned(self, state: &S, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, S, R> ParIteratorStringWithStateRuleOwned<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    #[inline(always)]
    fn par_iter_string_with_state_rule_owned(self, state: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateMutRuleOwned<S, R>
where
    R: FormatRuleMut<S>,
{
    fn par_iter_string_with_state_mut_rule_owned(self, initial_state: S, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, S, R> ParIteratorStringWithStateMutRuleOwned<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    #[inline(always)]
    fn par_iter_string_with_state_mut_rule_owned(
        self,
        mut initial_state: S,
        mut rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringRuleRef<'a, R>
where
    R: FormatRuleNoState<'a>,
{
    fn par_iter_string_rule_ref(self, rule: &'a R) -> String;
}

#[cfg(feature = "rayon")]
impl<'a, I, T, R> ParIteratorStringRuleRef<'a, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleNoState<'a>,
{
    #[inline(always)]
    fn par_iter_string_rule_ref(self, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringMutRuleRef<R>
where
    R: FormatRuleMutNoState,
{
    fn par_iter_string_mut_rule_ref(self, rule: &mut R) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, R> ParIteratorStringMutRuleRef<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMutNoState,
{
    #[inline(always)]
    fn par_iter_string_mut_rule_ref(self, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateRuleRef<S, R>
where
    R: FormatRule<S>,
{
    fn par_iter_string_with_state_rule_ref(self, state: &S, rule: &R) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, S, R> ParIteratorStringWithStateRuleRef<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRule<S>,
{
    #[inline(always)]
    fn par_iter_string_with_state_rule_ref(self, state: &S, rule: &R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(state, &s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateMutRuleRef<S, R>
where
    R: FormatRuleMut<S>,
{
    fn par_iter_string_with_state_mut_rule_ref(self, initial_state: S, rule: &mut R) -> String;
}

#[cfg(feature = "rayon")]
impl<I, T, S, R> ParIteratorStringWithStateMutRuleRef<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    R: FormatRuleMut<S>,
{
    #[inline(always)]
    fn par_iter_string_with_state_mut_rule_ref(self, mut initial_state: S, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let len = items.len();
        let mut result = String::new();
        #[unroll(full)]
        for (i, s) in items.into_iter().enumerate() {
            result.push_str(&rule.format(&mut initial_state, &s, i, len));
        }
        result
    }
}

#[cfg(feature = "rayon")]
impl<I, T> ExtendedDisplay for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    I: ParIteratorString,
    I: ParIteratorStringFn<fn(&str, usize, usize) -> String>,
    I: ParIteratorStringFnMut<fn(&str, usize, usize) -> String>,
    I: ParIteratorStringWithState<(), fn(&mut (), &str, usize, usize) -> String>,
    I: ParIteratorStringWithStateFn<(), fn(&(), &str, usize, usize) -> String>,
    I: ParIteratorStringWithStateFnPtr<()>,
    I: ParIteratorStringRuleOwned<fn(&str, usize, usize) -> String>,
    I: ParIteratorStringMutRuleOwned<fn(&str, usize, usize) -> String>,
    I: ParIteratorStringWithStateRuleOwned<(), fn(&(), &str, usize, usize) -> String>,
    I: ParIteratorStringWithStateMutRuleOwned<(), fn(&mut (), &str, usize, usize) -> String>,
    I: ParIteratorStringRuleRef<'static, fn(&str, usize, usize) -> String>,
    I: ParIteratorStringMutRuleRef<fn(&str, usize, usize) -> String>,
    I: ParIteratorStringWithStateRuleRef<(), fn(&(), &str, usize, usize) -> String>,
    I: ParIteratorStringWithStateMutRuleRef<(), fn(&mut (), &str, usize, usize) -> String>,
{
}

// ============================================================================
// #[cfg(feature = "dyn_async")] — DYN ASYNC (Box<dyn Future>)
// ============================================================================

// --- &self, !Send (compio/monoio) ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleNoStateAsync<'a, 'b>
where
    'b: 'a,
{
    fn format(
        &'a self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<'a, 'b, F> FormatRuleNoStateAsync<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFn(&'b str, usize, usize) -> String,
{
    fn format(
        &'a self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(self(value, index, length))
    }
}

// --- &self, Send (tokio/smol) ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleNoStateAsyncSend<'a, 'b>
where
    'b: 'a,
{
    fn format(
        &'a self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<'a, 'b, F> FormatRuleNoStateAsyncSend<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFn(&'b str, usize, usize) -> String + 'a,
    for<'c> <F as AsyncFnMut<(&'b str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn format(
        &'a self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(self(value, index, length))
    }
}

// --- self (owned), !Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleNoStateOwnedAsync {
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String>>;
}

#[cfg(feature = "dyn_async")]
impl<F> FormatRuleNoStateOwnedAsync for F
where
    F: 'static + AsyncFnOnce(String, usize, usize) -> String,
{
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String>> {
        Box::new(self(value, index, length))
    }
}

// --- self (owned), Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleNoStateOwnedAsyncSend {
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + Send>;
}

#[cfg(feature = "dyn_async")]
impl<F> FormatRuleNoStateOwnedAsyncSend for F
where
    F: 'static + AsyncFnOnce(String, usize, usize) -> String + Send,
    <F as AsyncFnOnce<(String, usize, usize)>>::CallOnceFuture: Send,
{
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + Send> {
        Box::new(self(value, index, length))
    }
}

// --- &mut self, !Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleMutNoStateAsync<'a, 'b>
where
    'b: 'a,
{
    fn format(
        &'a mut self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<'a, 'b, F> FormatRuleMutNoStateAsync<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFnMut(&'b str, usize, usize) -> String,
{
    fn format(
        &'a mut self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(self(value, index, length))
    }
}

// --- &mut self, Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleMutNoStateAsyncSend<'a, 'b>
where
    'b: 'a,
{
    fn format(
        &'a mut self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<'a, 'b, F> FormatRuleMutNoStateAsyncSend<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFnMut(&'b str, usize, usize) -> String + 'a,
    for<'c> <F as AsyncFnMut<(&'b str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn format(
        &'a mut self,
        value: &'b str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(self(value, index, length))
    }
}

// --- &self + State, !Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleAsync<S> {
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<S, F> FormatRuleAsync<S> for F
where
    F: AsyncFn(&S, &str, usize, usize) -> String,
{
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(self(state, value, index, length))
    }
}

// --- &self + State, Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleAsyncSend<S> {
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<S, F> FormatRuleAsyncSend<S> for F
where
    S: Sync,
    F: AsyncFn(&S, &str, usize, usize) -> String,
    for<'c> <F as AsyncFnMut<(&S, &str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(self(state, value, index, length))
    }
}

// --- &mut self + State, !Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleMutAsync<S> {
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<S, F> FormatRuleMutAsync<S> for F
where
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(self(state, value, index, length))
    }
}

// --- &mut self + State, Send ---
#[cfg(feature = "dyn_async")]
pub trait FormatRuleMutAsyncSend<S> {
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<S, F> FormatRuleMutAsyncSend<S> for F
where
    S: Send,
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
    for<'c> <F as AsyncFnMut<(&mut S, &str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(self(state, value, index, length))
    }
}

// --- Vec: &self + AsyncFn, !Send ---
#[cfg(feature = "dyn_async")]
pub trait VecStringFnAsync<'a, F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<'a, T, F> VecStringFnAsync<'a, F> for Vec<T>
where
    T: core::fmt::Display,
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        })
    }
}

// --- Vec: &self + AsyncFn, Send ---
#[cfg(feature = "dyn_async")]
pub trait VecStringFnAsyncSend<'a, F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<'a, T, F> VecStringFnAsyncSend<'a, F> for Vec<T>
where
    T: core::fmt::Display + Sync,
    F: AsyncFn(&str, usize, usize) -> String + Sync,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        })
    }
}

// --- Vec: &self + AsyncFnMut, !Send ---
#[cfg(feature = "dyn_async")]
pub trait VecStringFnMutAsync<'a, F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<'a, T, F> VecStringFnMutAsync<'a, F> for Vec<T>
where
    T: core::fmt::Display,
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        })
    }
}

// --- Vec: &self + AsyncFnMut, Send ---
#[cfg(feature = "dyn_async")]
pub trait VecStringFnMutAsyncSend<'a, F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<'a, T, F> VecStringFnMutAsyncSend<'a, F> for Vec<T>
where
    T: core::fmt::Display + Sync,
    F: AsyncFnMut(&str, usize, usize) -> String + Send,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn vec_string_async(
        &'a self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        })
    }
}

// --- Vec: &self + State + AsyncFnMut, !Send ---
#[cfg(feature = "dyn_async")]
pub trait VecStringWithStateAsync<'a, S, F>
where
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    fn vec_string_with_state_async(
        &'a self,
        initial_state: S,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<'a, T, S, F> VecStringWithStateAsync<'a, S, F> for Vec<T>
where
    T: core::fmt::Display,
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    fn vec_string_with_state_async(
        &'a self,
        mut initial_state: S,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(async move {
            let mut result = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                result.push_str(&format_rule(&mut initial_state, &s, i, len).await);
            }
            result
        })
    }
}

// --- Iterator: AsyncFn, !Send ---
#[cfg(feature = "dyn_async")]
pub trait IteratorStringFnAsync<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<I, T, F> IteratorStringFnAsync<F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

// --- Iterator: AsyncFn, Send ---
#[cfg(feature = "dyn_async")]
pub trait IteratorStringFnAsyncSend<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<I, T, F> IteratorStringFnAsyncSend<F> for I
where
    I: Iterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFn(&str, usize, usize) -> String + Sync,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

// --- Iterator: AsyncFnMut, !Send ---
#[cfg(feature = "dyn_async")]
pub trait IteratorStringFnMutAsync<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<I, T, F> IteratorStringFnMutAsync<F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

// --- Iterator: AsyncFnMut, Send ---
#[cfg(feature = "dyn_async")]
pub trait IteratorStringFnMutAsyncSend<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<I, T, F> IteratorStringFnMutAsyncSend<F> for I
where
    I: Iterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFnMut(&str, usize, usize) -> String + Send,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

// --- Iterator: State + AsyncFnMut, !Send ---
#[cfg(feature = "dyn_async")]
pub trait IteratorStringWithStateAsync<S, F>
where
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    fn iter_string_with_state_async<'a>(
        self,
        initial_state: S,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<I, T, S, F> IteratorStringWithStateAsync<S, F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    fn iter_string_with_state_async<'a>(
        self,
        mut initial_state: S,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&mut initial_state, &s, i, len).await);
            }
            result
        })
    }
}

// ============================================================================
// #[cfg(feature = "impl_async")] — IMPL ASYNC (через associated type, без Box)
// ============================================================================

// --- &self, !Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleNoStateImplAsync<'a, 'b>
where
    'b: 'a,
{
    type Future: core::future::Future<Output = String> + 'a;
    fn format(&'a self, value: &'b str, index: usize, length: usize) -> Self::Future;
}

#[cfg(feature = "impl_async")]
impl<'a, 'b, F> FormatRuleNoStateImplAsync<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFn(&'b str, usize, usize) -> String,
{
    type Future = impl core::future::Future<Output = String> + 'a;
    fn format(&'a self, value: &'b str, index: usize, length: usize) -> Self::Future {
        self(value, index, length)
    }
}

// --- &self, Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleNoStateImplAsyncSend<'a, 'b>
where
    'b: 'a,
{
    type Future: core::future::Future<Output = String> + 'a + Send;
    fn format(&'a self, value: &'b str, index: usize, length: usize) -> Self::Future;
}

#[cfg(feature = "impl_async")]
impl<'a, 'b, F> FormatRuleNoStateImplAsyncSend<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFn(&'b str, usize, usize) -> String,
    for<'c> <F as AsyncFnMut<(&'b str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future = impl core::future::Future<Output = String> + 'a + Send;
    fn format(&'a self, value: &'b str, index: usize, length: usize) -> Self::Future {
        self(value, index, length)
    }
}

// --- self (owned), !Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleNoStateOwnedImplAsync {
    type Future: core::future::Future<Output = String>;
    fn format(self, value: String, index: usize, length: usize) -> Self::Future;
}

#[cfg(feature = "impl_async")]
impl<F> FormatRuleNoStateOwnedImplAsync for F
where
    F: 'static + AsyncFnOnce(String, usize, usize) -> String,
{
    type Future = impl core::future::Future<Output = String>;
    fn format(self, value: String, index: usize, length: usize) -> Self::Future {
        self(value, index, length)
    }
}

// --- self (owned), Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleNoStateOwnedImplAsyncSend {
    type Future: core::future::Future<Output = String> + Send;
    fn format(self, value: String, index: usize, length: usize) -> Self::Future;
}

#[cfg(feature = "impl_async")]
impl<F> FormatRuleNoStateOwnedImplAsyncSend for F
where
    F: 'static + AsyncFnOnce(String, usize, usize) -> String + Send,
    <F as AsyncFnOnce<(String, usize, usize)>>::CallOnceFuture: Send,
{
    type Future = impl core::future::Future<Output = String> + Send;
    fn format(self, value: String, index: usize, length: usize) -> Self::Future {
        self(value, index, length)
    }
}

// --- &mut self, !Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleMutNoStateImplAsync<'a, 'b>
where
    'b: 'a,
{
    type Future: core::future::Future<Output = String> + 'a;
    fn format(&'a mut self, value: &'b str, index: usize, length: usize) -> Self::Future;
}

#[cfg(feature = "impl_async")]
impl<'a, 'b, F> FormatRuleMutNoStateImplAsync<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFnMut(&'b str, usize, usize) -> String,
{
    type Future = impl core::future::Future<Output = String> + 'a;
    fn format(&'a mut self, value: &'b str, index: usize, length: usize) -> Self::Future {
        self(value, index, length)
    }
}

// --- &mut self, Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleMutNoStateImplAsyncSend<'a, 'b>
where
    'b: 'a,
{
    type Future: core::future::Future<Output = String> + 'a + Send;
    fn format(&'a mut self, value: &'b str, index: usize, length: usize) -> Self::Future;
}

#[cfg(feature = "impl_async")]
impl<'a, 'b, F> FormatRuleMutNoStateImplAsyncSend<'a, 'b> for F
where
    'b: 'a,
    F: AsyncFnMut(&'b str, usize, usize) -> String,
    for<'c> <F as AsyncFnMut<(&'b str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future = impl core::future::Future<Output = String> + 'a + Send;
    fn format(&'a mut self, value: &'b str, index: usize, length: usize) -> Self::Future {
        self(value, index, length)
    }
}

// --- &self + State, !Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleImplAsync<S> {
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a;
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<S, F> FormatRuleImplAsync<S> for F
where
    F: AsyncFn(&S, &str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a;
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a> {
        self(state, value, index, length)
    }
}

// --- &self + State, Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleImplAsyncSend<S> {
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a;
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<S, F> FormatRuleImplAsyncSend<S> for F
where
    S: Sync,
    F: AsyncFn(&S, &str, usize, usize) -> String,
    for<'c> <F as AsyncFnMut<(&S, &str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a;
    fn format<'a>(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a> {
        self(state, value, index, length)
    }
}

// --- &mut self + State, !Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleMutImplAsync<S> {
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a;
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<S, F> FormatRuleMutImplAsync<S> for F
where
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a;
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a> {
        self(state, value, index, length)
    }
}

// --- &mut self + State, Send ---
#[cfg(feature = "impl_async")]
pub trait FormatRuleMutImplAsyncSend<S> {
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a;
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<S, F> FormatRuleMutImplAsyncSend<S> for F
where
    S: Send,
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
    for<'c> <F as AsyncFnMut<(&mut S, &str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a;
    fn format<'a>(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Self::Future<'a> {
        self(state, value, index, length)
    }
}

// --- Vec: &self + AsyncFn, !Send ---
#[cfg(feature = "impl_async")]
pub trait VecStringFnImplAsync<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<T, F> VecStringFnImplAsync<F> for Vec<T>
where
    T: core::fmt::Display,
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a F) -> Self::Future<'a> {
        async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        }
    }
}

// --- Vec: &self + AsyncFn, Send ---
#[cfg(feature = "impl_async")]
pub trait VecStringFnImplAsyncSend<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<T, F> VecStringFnImplAsyncSend<F> for Vec<T>
where
    T: core::fmt::Display + Sync,
    F: AsyncFn(&str, usize, usize) -> String + Sync,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a F) -> Self::Future<'a> {
        async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        }
    }
}

// --- Vec: &self + AsyncFnMut, !Send ---
#[cfg(feature = "impl_async")]
pub trait VecStringFnMutImplAsync<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a mut F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<T, F> VecStringFnMutImplAsync<F> for Vec<T>
where
    T: core::fmt::Display,
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a mut F) -> Self::Future<'a> {
        async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        }
    }
}

// --- Vec: &self + AsyncFnMut, Send ---
#[cfg(feature = "impl_async")]
pub trait VecStringFnMutImplAsyncSend<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a mut F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<T, F> VecStringFnMutImplAsyncSend<F> for Vec<T>
where
    T: core::fmt::Display + Sync,
    F: AsyncFnMut(&str, usize, usize) -> String + Send,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn vec_string_async<'a>(&'a self, format_rule: &'a mut F) -> Self::Future<'a> {
        async move {
            let mut string = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                string.push_str(&format_rule(&s, i, len).await);
            }
            string
        }
    }
}

// --- Vec: &self + State + AsyncFnMut, !Send ---
#[cfg(feature = "impl_async")]
pub trait VecStringWithStateImplAsync<S, F>
where
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a;
    fn vec_string_with_state_async<'a>(
        &'a self,
        initial_state: S,
        format_rule: &'a mut F,
    ) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<T, S, F> VecStringWithStateImplAsync<S, F> for Vec<T>
where
    T: core::fmt::Display,
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a;
    fn vec_string_with_state_async<'a>(
        &'a self,
        mut initial_state: S,
        format_rule: &'a mut F,
    ) -> Self::Future<'a> {
        async move {
            let mut result = String::new();
            let len = self.len();
            for (i, x) in self.iter().enumerate() {
                let s = format!("{}", x);
                result.push_str(&format_rule(&mut initial_state, &s, i, len).await);
            }
            result
        }
    }
}

// --- Iterator: AsyncFn, !Send ---
#[cfg(feature = "impl_async")]
pub trait IteratorStringFnImplAsync<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<I, T, F> IteratorStringFnImplAsync<F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

// --- Iterator: AsyncFn, Send ---
#[cfg(feature = "impl_async")]
pub trait IteratorStringFnImplAsyncSend<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<I, T, F> IteratorStringFnImplAsyncSend<F> for I
where
    I: Iterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFn(&str, usize, usize) -> String + Sync,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

// --- Iterator: AsyncFnMut, !Send ---
#[cfg(feature = "impl_async")]
pub trait IteratorStringFnMutImplAsync<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<I, T, F> IteratorStringFnMutImplAsync<F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

// --- Iterator: AsyncFnMut, Send ---
#[cfg(feature = "impl_async")]
pub trait IteratorStringFnMutImplAsyncSend<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<I, T, F> IteratorStringFnMutImplAsyncSend<F> for I
where
    I: Iterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFnMut(&str, usize, usize) -> String + Send,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

// --- Iterator: State + AsyncFnMut, !Send ---
#[cfg(feature = "impl_async")]
pub trait IteratorStringWithStateImplAsync<S, F>
where
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a;
    fn iter_string_with_state_async<'a>(
        self,
        initial_state: S,
        format_rule: &'a mut F,
    ) -> Self::Future<'a>;
}

#[cfg(feature = "impl_async")]
impl<I, T, S, F> IteratorStringWithStateImplAsync<S, F> for I
where
    I: Iterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFnMut(&mut S, &str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a;
    fn iter_string_with_state_async<'a>(
        self,
        mut initial_state: S,
        format_rule: &'a mut F,
    ) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&mut initial_state, &s, i, len).await);
            }
            result
        }
    }
}

// ============================================================================
// #[cfg(all(feature = "rayon", feature = "dyn_async"))] — RAYON DYN ASYNC
// ============================================================================

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
pub trait ParIteratorStringFnAsync<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<I, T, F> ParIteratorStringFnAsync<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
pub trait ParIteratorStringFnAsyncSend<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<I, T, F> ParIteratorStringFnAsyncSend<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFn(&str, usize, usize) -> String + Sync,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
pub trait ParIteratorStringFnMutAsync<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<I, T, F> ParIteratorStringFnMutAsync<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
pub trait ParIteratorStringFnMutAsyncSend<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<I, T, F> ParIteratorStringFnMutAsyncSend<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFnMut(&str, usize, usize) -> String + Send,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    fn par_iter_string_async<'a>(
        self,
        format_rule: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        })
    }
}

// ============================================================================
// #[cfg(all(feature = "rayon", feature = "impl_async"))] — RAYON IMPL ASYNC
// ============================================================================

#[cfg(all(feature = "rayon", feature = "impl_async"))]
pub trait ParIteratorStringFnImplAsync<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn par_iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a>;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<I, T, F> ParIteratorStringFnImplAsync<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn par_iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
pub trait ParIteratorStringFnImplAsyncSend<F>
where
    F: AsyncFn(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn par_iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a>;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<I, T, F> ParIteratorStringFnImplAsyncSend<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFn(&str, usize, usize) -> String + Sync,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn par_iter_string_async<'a>(self, format_rule: &'a F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
pub trait ParIteratorStringFnMutImplAsync<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn par_iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a>;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<I, T, F> ParIteratorStringFnMutImplAsync<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: core::fmt::Display,
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>
        = impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a;
    fn par_iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
pub trait ParIteratorStringFnMutImplAsyncSend<F>
where
    F: AsyncFnMut(&str, usize, usize) -> String,
{
    type Future<'a>: core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a;
    fn par_iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a>;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<I, T, F> ParIteratorStringFnMutImplAsyncSend<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T> + Send,
    T: core::fmt::Display + Send,
    F: AsyncFnMut(&str, usize, usize) -> String + Send,
    for<'c> <F as AsyncFnMut<(&str, usize, usize)>>::CallRefFuture<'c>: Send,
{
    type Future<'a>
        = Self
    where
        Self: 'a,
        F: 'a,
        Self: core::future::Future<Output = String> + 'a + Send;
    fn par_iter_string_async<'a>(self, format_rule: &'a mut F) -> Self::Future<'a> {
        async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let len = items.len();
            let mut result = String::new();
            for (i, s) in items.into_iter().enumerate() {
                result.push_str(&format_rule(&s, i, len).await);
            }
            result
        }
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    use core::future::Future;

    use super::*;
    use alloc::vec;
    #[cfg(not(feature = "std"))]
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    #[cfg(feature = "std")]
    use pollster::*;
    #[cfg(feature = "std")]
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn noop_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(p: *const ()) -> RawWaker {
            RawWaker::new(p, &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(core::ptr::null(), &VTABLE)
    }

    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    fn block_on<F: core::future::Future>(mut fut: F) -> F::Output {
        let mut fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };

        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut cx = Context::from_waker(&waker);

        loop {
            if let Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                return val;
            }
        }
    }

    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    fn block_on_dyn<T>(fut: Box<dyn Future<Output = T>>) -> T {
        let pin_future = Box::into_pin(fut);
        let fut = pin_future.as_mut();
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut cx = Context::from_waker(&waker);
    }

    // ========================================================================
    // SYNC ТЕСТЫ (всегда доступны)
    // ========================================================================

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
    // SYNC: ТЕСТЫ ВЛАДЕНИЕ
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
    // SYNC: ТЕСТЫ ПО ССЫЛКЕ
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
    // ExtendedDisplay
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

    // ========================================================================
    // #[cfg(feature = "rayon")] — SYNC RAYON ТЕСТЫ
    // ========================================================================

    #[cfg(feature = "rayon")]
    #[test]
    fn test_rayon_par_iter_string() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
        let numbers = vec![1, 2, 3];
        let s = numbers.into_par_iter().par_iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", s);
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn test_rayon_par_iter_string_fn() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
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
        let res = v.par_iter().par_iter_string(fmt);
        assert_eq!(res, "{10, 20, 30}");
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn test_rayon_extended_display() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
        fn takes_extended<T: ExtendedDisplay>(_x: T) {}
        let v = vec![1, 2, 3];
        takes_extended(v.par_iter());
        takes_extended(v.par_iter().map(|x| x * 2));
        takes_extended(v.into_par_iter());
    }

    // ========================================================================
    // #[cfg(feature = "dyn_async")] — DYN ASYNC ТЕСТЫ
    // ========================================================================

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_vec_string_fn_dyn_async() {
        let v = vec![1, 2, 3];
        let fmt = async |value: &str, index: usize, length: usize| -> String {
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
        let result = block_on(VecStringFnAsync::vec_string_async(&v, &fmt));
        assert_eq!(result, "<1, 2, 3>");
    }

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_iterator_string_fn_dyn_async() {
        let v = vec![10, 20, 30];
        let fmt = async |value: &str, index: usize, length: usize| -> String {
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
        let result = block_on(IteratorStringFnAsync::iter_string_async(v.iter(), &fmt));
        assert_eq!(result, "{10, 20, 30}");
    }

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_vec_string_fn_mut_dyn_async() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = async |value: &str, _index: usize, _length: usize| -> String {
            counter += 1;
            format!("[{}{}]", value, counter)
        };
        let result = block_on(VecStringFnMutAsync::vec_string_async(&v, &mut fmt));
        assert_eq!(result, "[a1][b2][c3]");
    }

    // ========================================================================
    // #[cfg(feature = "impl_async")] — IMPL ASYNC ТЕСТЫ
    // ========================================================================

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_vec_string_fn_impl_async() {
        let v = vec![1, 2, 3];
        let fmt = async |value: &str, index: usize, length: usize| -> String {
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
        let result = block_on(VecStringFnImplAsync::vec_string_async(&v, &fmt));
        assert_eq!(result, "<1, 2, 3>");
    }

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_iterator_string_fn_impl_async() {
        let v = vec![10, 20, 30];
        let fmt = async |value: &str, index: usize, length: usize| -> String {
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
        let result = block_on(IteratorStringFnImplAsync::iter_string_async(v.iter(), &fmt));
        assert_eq!(result, "{10, 20, 30}");
    }

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_vec_string_fn_mut_impl_async() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = async |value: &str, _index: usize, _length: usize| -> String {
            counter += 1;
            format!("[{}{}]", value, counter)
        };
        let result = block_on(VecStringFnMutImplAsync::vec_string_async(&v, &mut fmt));
        assert_eq!(result, "[a1][b2][c3]");
    }

    // ========================================================================
    // #[cfg(all(feature = "rayon", feature = "dyn_async"))] — RAYON DYN ASYNC ТЕСТЫ
    // ========================================================================

    #[cfg(all(feature = "rayon", feature = "dyn_async"))]
    #[test]
    fn test_rayon_par_iter_string_fn_dyn_async() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
        let v = vec![10, 20, 30];
        let fmt = async |value: &str, index: usize, length: usize| -> String {
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
        let f = ParIteratorStringFnAsync::par_iter_string_async(v.into_par_iter(), &fmt);
        let result = block_on(ParIteratorStringFnAsync::par_iter_string_async(
            v.into_par_iter(),
            &fmt,
        ));
        assert_eq!(result, "{10, 20, 30}");
    }

    // ========================================================================
    // #[cfg(all(feature = "rayon", feature = "impl_async"))] — RAYON IMPL ASYNC ТЕСТЫ
    // ========================================================================

    #[cfg(all(feature = "rayon", feature = "impl_async"))]
    #[test]
    fn test_rayon_par_iter_string_fn_impl_async() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
        let v = vec![10, 20, 30];
        let fmt = async |value: &str, index: usize, length: usize| -> String {
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
        let result = block_on(ParIteratorStringFnImplAsync::par_iter_string_async(
            v.into_par_iter(),
            &fmt,
        ));
        assert_eq!(result, "{10, 20, 30}");
    }
}
