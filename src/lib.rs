#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
#[cfg(any(feature = "dyn_async", feature = "impl_async"))]
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

// ============================================================================
// FormatElement - Unified formatting trait
// ============================================================================
pub trait FormatElement {
    fn format_element(&self) -> String;
}

impl<T: core::fmt::Display> FormatElement for T {
    fn format_element(&self) -> String {
        format!("{}", self)
    }
}

impl<T> FormatElement for Vec<T> 
where 
    Vec<T>: VecString 
{
    fn format_element(&self) -> String {
        self.vec_string(DEFAULT_FORMAT_RULE)
    }
}

#[macro_export]
macro_rules! impl_format_element_for_vec {
    ($t:ty) => {
        impl $crate::FormatElement for $t {
            fn format_element(&self) -> String {
                $crate::VecString::vec_string(self, $crate::DEFAULT_FORMAT_RULE)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_format_element_for_iter {
    ($t:ty) => {
        impl $crate::FormatElement for $t 
        where 
            Self: Clone 
        {
            fn format_element(&self) -> String {
                $crate::IteratorString::iter_string(self.clone(), $crate::DEFAULT_FORMAT_RULE)
            }
        }
    };
}

// ============================================================================
// StableIter
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait StableIter: Iterator {}
impl<'a, T> StableIter for core::slice::Iter<'a, T> {}
impl<'a, T> StableIter for core::slice::IterMut<'a, T> {}
impl<T> StableIter for alloc::vec::IntoIter<T> {}
impl<T, const N: usize> StableIter for core::array::IntoIter<T, N> {}
impl<I, F, B> StableIter for core::iter::Map<I, F>
where
    I: StableIter,
    F: FnMut(I::Item) -> B,
{
}
impl<I, P> StableIter for core::iter::Filter<I, P>
where
    I: StableIter,
    P: FnMut(&I::Item) -> bool,
{
}
impl<I, F, B> StableIter for core::iter::FilterMap<I, F>
where
    I: StableIter,
    F: FnMut(I::Item) -> Option<B>,
{
}
impl<I> StableIter for core::iter::Take<I> where I: StableIter {}
impl<I> StableIter for core::iter::Skip<I> where I: StableIter {}
impl<I, P> StableIter for core::iter::TakeWhile<I, P>
where
    I: StableIter,
    P: FnMut(&I::Item) -> bool,
{
}
impl<I, P> StableIter for core::iter::SkipWhile<I, P>
where
    I: StableIter,
    P: FnMut(&I::Item) -> bool,
{
}
impl<'a, I, T> StableIter for core::iter::Cloned<I>
where
    I: StableIter<Item = &'a T>,
    T: Clone + 'a,
{
}
impl<'a, I, T> StableIter for core::iter::Copied<I>
where
    I: StableIter<Item = &'a T>,
    T: Copy + 'a,
{
}
impl<I, U> StableIter for core::iter::Chain<I, U>
where
    I: StableIter,
    U: StableIter<Item = I::Item>,
{
}
impl<I, U> StableIter for core::iter::Zip<I, U>
where
    I: StableIter,
    U: StableIter,
{
}
impl<I> StableIter for core::iter::Enumerate<I> where I: StableIter {}
impl<I, U, F> StableIter for core::iter::FlatMap<I, U, F>
where
    I: StableIter,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
}
impl<I> StableIter for core::iter::Flatten<I>
where
    I: StableIter,
    I::Item: IntoIterator,
{
}
impl<I> StableIter for core::iter::Fuse<I> where I: StableIter {}
impl<I> StableIter for core::iter::Peekable<I> where I: StableIter {}
impl<I> StableIter for core::iter::StepBy<I> where I: StableIter {}
impl<I> StableIter for core::iter::Cycle<I> where I: StableIter + Clone {}
impl<I> StableIter for core::iter::Rev<I> where I: StableIter + DoubleEndedIterator {}
impl<T> StableIter for core::iter::Once<T> {}
impl<T> StableIter for core::iter::Empty<T> {}
impl<F> StableIter for core::iter::RepeatWith<F> where F: FnMut() {}
impl<T> StableIter for core::iter::Repeat<T> where T: Clone {}
impl<I, F> StableIter for core::iter::Inspect<I, F>
where
    I: StableIter,
    F: FnMut(&I::Item),
{
}
impl<I, St, F, B> StableIter for core::iter::Scan<I, St, F>
where
    I: StableIter,
    F: FnMut(&mut St, I::Item) -> Option<B>,
{
}

// ============================================================================
// ExtendedDisplay
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ExtendedDisplay {}
impl<T> ExtendedDisplay for Vec<T>
where
    T: FormatElement,
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
impl<I> ExtendedDisplay for I
where
    I: StableIter,
    I::Item: FormatElement,
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
// SYNC ТРЕЙТЫ ПРАВИЛ
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoState<'a> {
    fn format(&'a self, value: &str, index: usize, length: usize) -> String;
}
impl<'a, F: Fn(&str, usize, usize) -> String> FormatRuleNoState<'a> for F {
    fn format(&'a self, v: &str, i: usize, l: usize) -> String {
        (self)(v, i, l)
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateOwned {
    fn format(self, value: &str, index: usize, length: usize) -> String;
}
impl<F: Fn(&str, usize, usize) -> String> FormatRuleNoStateOwned for F {
    fn format(self, v: &str, i: usize, l: usize) -> String {
        (self)(v, i, l)
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutNoState {
    fn format(&mut self, value: &str, index: usize, length: usize) -> String;
}
impl<F: FnMut(&str, usize, usize) -> String> FormatRuleMutNoState for F {
    fn format(&mut self, v: &str, i: usize, l: usize) -> String {
        (self)(v, i, l)
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRule<S> {
    fn format(&self, state: &S, value: &str, index: usize, length: usize) -> String;
}
impl<S, F: Fn(&S, &str, usize, usize) -> String> FormatRule<S> for F {
    fn format(&self, s: &S, v: &str, i: usize, l: usize) -> String {
        (self)(s, v, i, l)
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMut<S> {
    fn format(&mut self, state: &mut S, value: &str, index: usize, length: usize) -> String;
}
impl<S, F: FnMut(&mut S, &str, usize, usize) -> String> FormatRuleMut<S> for F {
    fn format(&mut self, s: &mut S, v: &str, i: usize, l: usize) -> String {
        (self)(s, v, i, l)
    }
}

// ============================================================================
// SYNC VecString* / IteratorString* (collecting)
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecString {
    fn vec_string(&self, format_rule: FormatRuleFn) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFn<F: Fn(&str, usize, usize) -> String> {
    fn vec_string_fn(&self, format_rule: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn vec_string_fn_mut(&self, format_rule: F) -> String;
}

impl<T: FormatElement> VecString for Vec<T> {
    fn vec_string(&self, f: FormatRuleFn) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&x.format_element(), i, l));
        }
        s
    }
}

impl<T: FormatElement, F: Fn(&str, usize, usize) -> String> VecStringFn<F> for Vec<T> {
    fn vec_string_fn(&self, f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&x.format_element(), i, l));
        }
        s
    }
}

impl<T: FormatElement, F: FnMut(&str, usize, usize) -> String> VecStringFnMut<F> for Vec<T> {
    fn vec_string_fn_mut(&self, mut f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&x.format_element(), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorString {
    fn iter_string(self, format_rule: FormatRuleFn) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFn<F: Fn(&str, usize, usize) -> String> {
    fn iter_string_fn(self, format_rule: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn iter_string_fn_mut(self, format_rule: F) -> String;
}

impl<I: StableIter> IteratorString for I
where
    I::Item: FormatElement,
{
    fn iter_string(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

impl<I: StableIter, F: Fn(&str, usize, usize) -> String> IteratorStringFn<F> for I
where
    I::Item: FormatElement,
{
    fn iter_string_fn(self, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

impl<I: StableIter, F: FnMut(&str, usize, usize) -> String> IteratorStringFnMut<F> for I
where
    I::Item: FormatElement,
{
    fn iter_string_fn_mut(self, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn vec_string_with_state(&self, st: S, f: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn iter_string_with_state(self, st: S, f: F) -> String;
}

impl<T: FormatElement, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    VecStringWithState<S, F> for Vec<T>
{
    fn vec_string_with_state(&self, mut st: S, mut f: F) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.format_element();
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

impl<I: StableIter, S, F: FnMut(&mut S, &str, usize, usize) -> String> IteratorStringWithState<S, F>
    for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state(self, mut st: S, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn vec_string_with_state_fn(&self, st: &S, f: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn iter_string_with_state_fn(self, st: &S, f: F) -> String;
}

impl<T: FormatElement, S, F: Fn(&S, &str, usize, usize) -> String> VecStringWithStateFn<S, F>
    for Vec<T>
{
    fn vec_string_with_state_fn(&self, st: &S, f: F) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.format_element();
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

impl<I: StableIter, S, F: Fn(&S, &str, usize, usize) -> String> IteratorStringWithStateFn<S, F>
    for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_fn(self, st: &S, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateFnPtr<S> {
    fn vec_string_with_state_fn_ptr(
        &self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFnPtr<S> {
    fn iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

impl<T: FormatElement, S> VecStringWithStateFnPtr<S> for Vec<T> {
    fn vec_string_with_state_fn_ptr(
        &self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.format_element();
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

impl<I: StableIter, S> IteratorStringWithStateFnPtr<S> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// SYNC RuleOwned / MutRuleOwned / RuleRef / MutRuleRef (collecting)
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn vec_string_rule_owned(self, rule: R) -> String;
}

impl<T: FormatElement, R: FormatRuleNoStateOwned + Clone> VecStringRuleOwned<R> for Vec<T> {
    fn vec_string_rule_owned(self, rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.clone().format(&x.format_element(), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_owned(&self, rule: R) -> String;
}

impl<T: FormatElement, R: FormatRuleMutNoState> VecStringMutRuleOwned<R> for Vec<T> {
    fn vec_string_mut_rule_owned(&self, mut rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&x.format_element(), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn iter_string_rule_owned(self, rule: R) -> String;
}

impl<I: StableIter, R: FormatRuleNoStateOwned + Clone> IteratorStringRuleOwned<R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_rule_owned(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.clone().format(&s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_owned(self, rule: R) -> String;
}

impl<I: StableIter, R: FormatRuleMutNoState> IteratorStringMutRuleOwned<R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_mut_rule_owned(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_owned(&self, st: &S, rule: R) -> String;
}

impl<T: FormatElement, S, R: FormatRule<S>> VecStringWithStateRuleOwned<S, R> for Vec<T> {
    fn vec_string_with_state_rule_owned(&self, st: &S, rule: R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String;
}

impl<I: StableIter, S, R: FormatRule<S>> IteratorStringWithStateRuleOwned<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_owned(&self, st: S, rule: R) -> String;
}

impl<T: FormatElement, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleOwned<S, R>
    for Vec<T>
{
    fn vec_string_with_state_mut_rule_owned(&self, mut st: S, mut rule: R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_owned(self, st: S, rule: R) -> String;
}

impl<I: StableIter, S, R: FormatRuleMut<S>> IteratorStringWithStateMutRuleOwned<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_mut_rule_owned(self, mut st: S, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringRuleRef<'a, R: FormatRuleNoState<'a>> {
    fn vec_string_rule_ref(&self, rule: &'a R) -> String;
}

impl<'a, T: FormatElement, R: FormatRuleNoState<'a>> VecStringRuleRef<'a, R> for Vec<T> {
    fn vec_string_rule_ref(&self, rule: &'a R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&x.format_element(), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringMutRuleRef<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String;
}

impl<T: FormatElement, R: FormatRuleMutNoState> VecStringMutRuleRef<R> for Vec<T> {
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&x.format_element(), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleRef<'a, R: FormatRuleNoState<'a>> {
    fn iter_string_rule_ref(self, rule: &'a R) -> String;
}

impl<'a, I: StableIter, R: FormatRuleNoState<'a>> IteratorStringRuleRef<'a, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_rule_ref(self, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleRef<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_ref(self, rule: &mut R) -> String;
}

impl<I: StableIter, R: FormatRuleMutNoState> IteratorStringMutRuleRef<R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_mut_rule_ref(self, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_ref(&self, st: &S, rule: &R) -> String;
}

impl<T: FormatElement, S, R: FormatRule<S>> VecStringWithStateRuleRef<S, R> for Vec<T> {
    fn vec_string_with_state_rule_ref(&self, st: &S, rule: &R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String;
}

impl<I: StableIter, S, R: FormatRule<S>> IteratorStringWithStateRuleRef<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_ref(&self, st: S, rule: &mut R) -> String;
}

impl<T: FormatElement, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleRef<S, R> for Vec<T> {
    fn vec_string_with_state_mut_rule_ref(&self, mut st: S, rule: &mut R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_ref(self, st: S, rule: &mut R) -> String;
}

impl<I: StableIter, S, R: FormatRuleMut<S>> IteratorStringWithStateMutRuleRef<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_mut_rule_ref(self, mut st: S, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// SYNC Exact-size traits (no allocation)
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringExact {
    fn iter_string_exact(self, format_rule: FormatRuleFn) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnExact<F: Fn(&str, usize, usize) -> String> {
    fn iter_string_fn_exact(self, format_rule: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutExact<F: FnMut(&str, usize, usize) -> String> {
    fn iter_string_fn_mut_exact(self, format_rule: F) -> String;
}

impl<I: StableIter + ExactSizeIterator> IteratorStringExact for I
where
    I::Item: FormatElement,
{
    fn iter_string_exact(self, f: FormatRuleFn) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&x.format_element(), i, l));
        }
        r
    }
}

impl<I: StableIter + ExactSizeIterator, F: Fn(&str, usize, usize) -> String>
    IteratorStringFnExact<F> for I
where
    I::Item: FormatElement,
{
    fn iter_string_fn_exact(self, f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&x.format_element(), i, l));
        }
        r
    }
}

impl<I: StableIter + ExactSizeIterator, F: FnMut(&str, usize, usize) -> String>
    IteratorStringFnMutExact<F> for I
where
    I::Item: FormatElement,
{
    fn iter_string_fn_mut_exact(self, mut f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&x.format_element(), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateExact<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn iter_string_with_state_exact(self, st: S, f: F) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    IteratorStringWithStateExact<S, F> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_exact(self, mut st: S, mut f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.format_element();
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFnExact<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn iter_string_with_state_fn_exact(self, st: &S, f: F) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, F: Fn(&S, &str, usize, usize) -> String>
    IteratorStringWithStateFnExact<S, F> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_fn_exact(self, st: &S, f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.format_element();
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFnPtrExact<S> {
    fn iter_string_with_state_fn_ptr_exact(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, S> IteratorStringWithStateFnPtrExact<S> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_fn_ptr_exact(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.format_element();
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleOwnedExact<R: FormatRuleNoStateOwned> {
    fn iter_string_rule_owned_exact(self, rule: R) -> String;
}

impl<I: StableIter + ExactSizeIterator, R: FormatRuleNoStateOwned + Clone>
    IteratorStringRuleOwnedExact<R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_rule_owned_exact(self, rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.clone().format(&x.format_element(), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleOwnedExact<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_owned_exact(self, rule: R) -> String;
}

impl<I: StableIter + ExactSizeIterator, R: FormatRuleMutNoState> IteratorStringMutRuleOwnedExact<R>
    for I
where
    I::Item: FormatElement,
{
    fn iter_string_mut_rule_owned_exact(self, mut rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&x.format_element(), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleOwnedExact<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_owned_exact(self, st: &S, rule: R) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRule<S>>
    IteratorStringWithStateRuleOwnedExact<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_rule_owned_exact(self, st: &S, rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleOwnedExact<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_owned_exact(self, st: S, rule: R) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRuleMut<S>>
    IteratorStringWithStateMutRuleOwnedExact<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_mut_rule_owned_exact(self, mut st: S, mut rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleRefExact<'a, R: FormatRuleNoState<'a>> {
    fn iter_string_rule_ref_exact(self, rule: &'a R) -> String;
}

impl<'a, I: StableIter + ExactSizeIterator, R: FormatRuleNoState<'a>>
    IteratorStringRuleRefExact<'a, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_rule_ref_exact(self, rule: &'a R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&x.format_element(), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleRefExact<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_ref_exact(self, rule: &mut R) -> String;
}

impl<I: StableIter + ExactSizeIterator, R: FormatRuleMutNoState> IteratorStringMutRuleRefExact<R>
    for I
where
    I::Item: FormatElement,
{
    fn iter_string_mut_rule_ref_exact(self, rule: &mut R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&x.format_element(), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleRefExact<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_ref_exact(self, st: &S, rule: &R) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRule<S>>
    IteratorStringWithStateRuleRefExact<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_rule_ref_exact(self, st: &S, rule: &R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleRefExact<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_ref_exact(self, st: S, rule: &mut R) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRuleMut<S>>
    IteratorStringWithStateMutRuleRefExact<S, R> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_mut_rule_ref_exact(self, mut st: S, rule: &mut R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.format_element();
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// RAYON SYNC
// ============================================================================
#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorString {
    fn par_iter_string(self, f: FormatRuleFn) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement> ParIteratorString for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(&s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFn<F: Fn(&str, usize, usize) -> String> {
    fn par_iter_string_fn(self, f: F) -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        F: Fn(&str, usize, usize) -> String + Sync,
    > ParIteratorStringFn<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn(self, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(&s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn par_iter_string_fn_mut(self, f: F) -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        F: FnMut(&str, usize, usize) -> String,
    > ParIteratorStringFnMut<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn_mut(self, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnPtr {
    fn par_iter_string_fn_ptr(self, f: FormatRuleFn) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement> ParIteratorStringFnPtr for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn_ptr(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(&s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn par_iter_string_with_state(self, st: S, f: F) -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> String,
    > ParIteratorStringWithState<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state(self, mut st: S, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn par_iter_string_with_state_fn(self, st: &S, f: F) -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        S: Sync,
        F: Fn(&S, &str, usize, usize) -> String + Sync,
    > ParIteratorStringWithStateFn<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_fn(self, st: &S, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(st, &s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateFnPtr<S> {
    fn par_iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement, S: Sync>
    ParIteratorStringWithStateFnPtr<S> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(st, &s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn par_iter_string_rule_owned(self, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        R: FormatRuleNoStateOwned + Clone + Sync,
    > ParIteratorStringRuleOwned<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_rule_owned(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.clone().format(&s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn par_iter_string_mut_rule_owned(self, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement, R: FormatRuleMutNoState>
    ParIteratorStringMutRuleOwned<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_mut_rule_owned(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn par_iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement, S: Sync, R: FormatRule<S> + Sync>
    ParIteratorStringWithStateRuleOwned<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.format(st, &s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn par_iter_string_with_state_mut_rule_owned(self, st: S, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement, S, R: FormatRuleMut<S>>
    ParIteratorStringWithStateMutRuleOwned<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_mut_rule_owned(self, mut st: S, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringRuleRef<'a, R: FormatRuleNoState<'a>> {
    fn par_iter_string_rule_ref(self, rule: &'a R) -> String;
}

#[cfg(feature = "rayon")]
impl<
        'a,
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        R: FormatRuleNoState<'a> + Sync,
    > ParIteratorStringRuleRef<'a, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_rule_ref(self, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.format(&s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringMutRuleRef<R: FormatRuleMutNoState> {
    fn par_iter_string_mut_rule_ref(self, rule: &mut R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement, R: FormatRuleMutNoState>
    ParIteratorStringMutRuleRef<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_mut_rule_ref(self, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn par_iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement, S: Sync, R: FormatRule<S> + Sync>
    ParIteratorStringWithStateRuleRef<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.format(st, &s, i, l))
            .reduce(
                || String::new(),
                |mut a, c| {
                    a.push_str(&c);
                    a
                },
            )
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn par_iter_string_with_state_mut_rule_ref(self, st: S, rule: &mut R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: FormatElement, S, R: FormatRuleMut<S>>
    ParIteratorStringWithStateMutRuleRef<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_mut_rule_ref(self, mut st: S, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| x.format_element()).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// DYN ASYNC: Format Rule traits
// ============================================================================
#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateAsync<'a> {
    fn format(
        &'a self,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<'a, F: Fn(&str, usize, usize) -> Box<dyn core::future::Future<Output = String> + 'a>>
    FormatRuleNoStateAsync<'a> for F
{
    fn format(
        &'a self,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateAsyncSend<'a> {
    fn format(
        &'a self,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        F: Fn(&str, usize, usize) -> Box<dyn core::future::Future<Output = String> + 'a + Send>,
    > FormatRuleNoStateAsyncSend<'a> for F
{
    fn format(
        &'a self,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateOwnedAsync {
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String>>;
}

#[cfg(feature = "dyn_async")]
impl<F: FnOnce(String, usize, usize) -> Box<dyn core::future::Future<Output = String>>>
    FormatRuleNoStateOwnedAsync for F
{
    fn format(
        self,
        v: String,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String>> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateOwnedAsyncSend {
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + Send>;
}

#[cfg(feature = "dyn_async")]
impl<F: FnOnce(String, usize, usize) -> Box<dyn core::future::Future<Output = String> + Send>>
    FormatRuleNoStateOwnedAsyncSend for F
{
    fn format(
        self,
        v: String,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + Send> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutNoStateAsync<'a> {
    fn format(
        &'a mut self,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<'a, F: FnMut(&str, usize, usize) -> Box<dyn core::future::Future<Output = String> + 'a>>
    FormatRuleMutNoStateAsync<'a> for F
{
    fn format(
        &'a mut self,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutNoStateAsyncSend<'a> {
    fn format(
        &'a mut self,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        F: FnMut(&str, usize, usize) -> Box<dyn core::future::Future<Output = String> + 'a + Send>,
    > FormatRuleMutNoStateAsyncSend<'a> for F
{
    fn format(
        &'a mut self,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleAsync<'a, S> {
    fn format(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        S,
        F: Fn(&'a S, &str, usize, usize) -> Box<dyn core::future::Future<Output = String> + 'a>,
    > FormatRuleAsync<'a, S> for F
{
    fn format(
        &'a self,
        s: &'a S,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleAsyncSend<'a, S> {
    fn format(
        &'a self,
        state: &'a S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        S,
        F: Fn(&'a S, &str, usize, usize) -> Box<dyn core::future::Future<Output = String> + 'a + Send>,
    > FormatRuleAsyncSend<'a, S> for F
{
    fn format(
        &'a self,
        s: &'a S,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutAsync<'a, S> {
    fn format(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        S,
        F: FnMut(&'a mut S, &str, usize, usize) -> Box<dyn core::future::Future<Output = String> + 'a>,
    > FormatRuleMutAsync<'a, S> for F
{
    fn format(
        &'a mut self,
        s: &'a mut S,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutAsyncSend<'a, S> {
    fn format(
        &'a mut self,
        state: &'a mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        S,
        F: FnMut(
            &'a mut S,
            &str,
            usize,
            usize,
        ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>,
    > FormatRuleMutAsyncSend<'a, S> for F
{
    fn format(
        &'a mut self,
        s: &'a mut S,
        v: &str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleFnPtrAsync {
    fn format<'a>(
        &'a self,
        value: &'a str,
        index: usize,
        length: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl FormatRuleFnPtrAsync
    for fn(&str, usize, usize) -> Box<dyn core::future::Future<Output = String> + '_>
{
    fn format<'a>(
        &'a self,
        v: &'a str,
        i: usize,
        l: usize,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        (self)(v, i, l)
    }
}

// ============================================================================
// DYN ASYNC: Vec (unchanged, already collecting)
// ============================================================================
#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnAsync<
    'a,
    F: Fn(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn vec_string_async_fn(
        &'a self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        T: FormatElement,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > VecStringFnAsync<'a, F, Fut> for Vec<T>
{
    fn vec_string_async_fn(
        &'a self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnAsyncSend<
    'a,
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + 'a + Send,
>
{
    fn vec_string_async_fn(
        &'a self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        T: FormatElement + Sync,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > VecStringFnAsyncSend<'a, F, Fut> for Vec<T>
{
    fn vec_string_async_fn(
        &'a self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnMutAsync<
    'a,
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn vec_string_async_fn_mut(
        &'a self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        T: FormatElement,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > VecStringFnMutAsync<'a, F, Fut> for Vec<T>
{
    fn vec_string_async_fn_mut(
        &'a self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnMutAsyncSend<
    'a,
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + 'a + Send,
>
{
    fn vec_string_async_fn_mut(
        &'a self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        T: FormatElement + Sync,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > VecStringFnMutAsyncSend<'a, F, Fut> for Vec<T>
{
    fn vec_string_async_fn_mut(
        &'a self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send> {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateAsync<
    'a,
    S: 'a,
    F: FnMut(&mut S, &str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn vec_string_with_state_async(
        &'a self,
        st: S,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        T: FormatElement,
        S: 'a,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > VecStringWithStateAsync<'a, S, F, Fut> for Vec<T>
{
    fn vec_string_with_state_async(
        &'a self,
        mut st: S,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a> {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        })
    }
}

// ============================================================================
// DYN ASYNC: Iterator – COLLECTING versions
// ============================================================================
#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnAsync<
    'a,
    F: Fn(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn iter_string_async_fn(self, f: &'a F) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringFnAsync<'a, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn(self, f: &'a F) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnAsyncSend<
    'a,
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + 'a + Send,
>
{
    fn iter_string_async_fn(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > IteratorStringFnAsyncSend<'a, F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutAsync<
    'a,
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn iter_string_async_fn_mut(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringFnMutAsync<'a, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn_mut(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutAsyncSend<
    'a,
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + 'a + Send,
>
{
    fn iter_string_async_fn_mut(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > IteratorStringFnMutAsyncSend<'a, F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn_mut(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateAsync<
    'a,
    S: 'a,
    F: FnMut(&mut S, &str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn iter_string_with_state_async(
        self,
        st: S,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter,
        S: 'a,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringWithStateAsync<'a, S, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_async(
        self,
        mut st: S,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        })
    }
}

// ============================================================================
// DYN ASYNC: Iterator – EXACT versions (no collect)
// ============================================================================
#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnAsyncExact<
    'a,
    F: Fn(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn iter_string_async_fn_exact(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter + ExactSizeIterator,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringFnAsyncExact<'a, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn_exact(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnAsyncSendExact<
    'a,
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + 'a + Send,
>
{
    fn iter_string_async_fn_exact(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter + ExactSizeIterator + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > IteratorStringFnAsyncSendExact<'a, F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn_exact(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutAsyncExact<
    'a,
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn iter_string_async_fn_mut_exact(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter + ExactSizeIterator,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringFnMutAsyncExact<'a, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn_mut_exact(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutAsyncSendExact<
    'a,
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + 'a + Send,
>
{
    fn iter_string_async_fn_mut_exact(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter + ExactSizeIterator + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > IteratorStringFnMutAsyncSendExact<'a, F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn_mut_exact(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
    {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(feature = "dyn_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateAsyncExact<
    'a,
    S: 'a,
    F: FnMut(&mut S, &str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String> + 'a,
>
{
    fn iter_string_with_state_async_exact(
        self,
        st: S,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(feature = "dyn_async")]
impl<
        'a,
        I: StableIter + ExactSizeIterator,
        S: 'a,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringWithStateAsyncExact<'a, S, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_async_exact(
        self,
        mut st: S,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        })
    }
}

// ============================================================================
// IMPL ASYNC: Format Rule traits
// ============================================================================
#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateImplAsync<Fut: core::future::Future<Output = String>> {
    fn format(
        &self,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String>;
}

#[cfg(feature = "impl_async")]
impl<F: Fn(&str, usize, usize) -> Fut, Fut: core::future::Future<Output = String>>
    FormatRuleNoStateImplAsync<Fut> for F
{
    fn format(&self, v: &str, i: usize, l: usize) -> impl core::future::Future<Output = String> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateImplAsyncSend<Fut: core::future::Future<Output = String> + Send> {
    fn format(
        &self,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}

#[cfg(feature = "impl_async")]
impl<
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > FormatRuleNoStateImplAsyncSend<Fut> for F
{
    fn format(
        &self,
        v: &str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> + Send {
        (self)(v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateOwnedImplAsync<Fut: core::future::Future<Output = String>> {
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String>;
}

#[cfg(feature = "impl_async")]
impl<F: FnOnce(String, usize, usize) -> Fut, Fut: core::future::Future<Output = String>>
    FormatRuleNoStateOwnedImplAsync<Fut> for F
{
    fn format(self, v: String, i: usize, l: usize) -> impl core::future::Future<Output = String> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoStateOwnedImplAsyncSend<Fut: core::future::Future<Output = String> + Send> {
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}

#[cfg(feature = "impl_async")]
impl<
        F: FnOnce(String, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > FormatRuleNoStateOwnedImplAsyncSend<Fut> for F
{
    fn format(
        self,
        v: String,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> + Send {
        (self)(v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutNoStateImplAsync<Fut: core::future::Future<Output = String>> {
    fn format(
        &mut self,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String>;
}

#[cfg(feature = "impl_async")]
impl<F: FnMut(&str, usize, usize) -> Fut, Fut: core::future::Future<Output = String>>
    FormatRuleMutNoStateImplAsync<Fut> for F
{
    fn format(
        &mut self,
        v: &str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> {
        (self)(v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutNoStateImplAsyncSend<Fut: core::future::Future<Output = String> + Send> {
    fn format(
        &mut self,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}

#[cfg(feature = "impl_async")]
impl<
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > FormatRuleMutNoStateImplAsyncSend<Fut> for F
{
    fn format(
        &mut self,
        v: &str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> + Send {
        (self)(v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleImplAsync<S, Fut: core::future::Future<Output = String>> {
    fn format(
        &self,
        state: &S,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String>;
}

#[cfg(feature = "impl_async")]
impl<S, F: Fn(&S, &str, usize, usize) -> Fut, Fut: core::future::Future<Output = String>>
    FormatRuleImplAsync<S, Fut> for F
{
    fn format(
        &self,
        s: &S,
        v: &str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleImplAsyncSend<S, Fut: core::future::Future<Output = String> + Send> {
    fn format(
        &self,
        state: &S,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}

#[cfg(feature = "impl_async")]
impl<
        S: Sync,
        F: Fn(&S, &str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > FormatRuleImplAsyncSend<S, Fut> for F
{
    fn format(
        &self,
        s: &S,
        v: &str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> + Send {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutImplAsync<S, Fut: core::future::Future<Output = String>> {
    fn format(
        &mut self,
        state: &mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String>;
}

#[cfg(feature = "impl_async")]
impl<
        S,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > FormatRuleMutImplAsync<S, Fut> for F
{
    fn format(
        &mut self,
        s: &mut S,
        v: &str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleMutImplAsyncSend<S, Fut: core::future::Future<Output = String> + Send> {
    fn format(
        &mut self,
        state: &mut S,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}

#[cfg(feature = "impl_async")]
impl<
        S: Send,
        F: FnMut(&mut S, &str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > FormatRuleMutImplAsyncSend<S, Fut> for F
{
    fn format(
        &mut self,
        s: &mut S,
        v: &str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> + Send {
        (self)(s, v, i, l)
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleFnPtrImplAsync<Fut: core::future::Future<Output = String>> {
    fn format<'a>(
        &'a self,
        value: &'a str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + 'a;
}

#[cfg(feature = "impl_async")]
impl<Fut: core::future::Future<Output = String>> FormatRuleFnPtrImplAsync<Fut>
    for fn(&str, usize, usize) -> Fut
{
    fn format<'a>(
        &'a self,
        v: &'a str,
        i: usize,
        l: usize,
    ) -> impl core::future::Future<Output = String> + 'a {
        (self)(v, i, l)
    }
}

// ============================================================================
// IMPL ASYNC: Vec (collecting, same as before)
// ============================================================================
#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnImplAsync<
    F: Fn(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn vec_string_async_fn<'a>(
        &'a self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        T: FormatElement,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > VecStringFnImplAsync<F, Fut> for Vec<T>
{
    fn vec_string_async_fn<'a>(
        &'a self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnImplAsyncSend<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn vec_string_async_fn<'a>(
        &'a self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        T: FormatElement + Sync,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > VecStringFnImplAsyncSend<F, Fut> for Vec<T>
{
    fn vec_string_async_fn<'a>(
        &'a self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnMutImplAsync<
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn vec_string_async_fn_mut<'a>(
        &'a self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        T: FormatElement,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > VecStringFnMutImplAsync<F, Fut> for Vec<T>
{
    fn vec_string_async_fn_mut<'a>(
        &'a self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnMutImplAsyncSend<
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn vec_string_async_fn_mut<'a>(
        &'a self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        T: FormatElement + Sync,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > VecStringFnMutImplAsyncSend<F, Fut> for Vec<T>
{
    fn vec_string_async_fn_mut<'a>(
        &'a self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateImplAsync<
    S,
    F: FnMut(&mut S, &str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn vec_string_with_state_async<'a>(
        &'a self,
        st: S,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        T: FormatElement,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > VecStringWithStateImplAsync<S, F, Fut> for Vec<T>
{
    fn vec_string_with_state_async<'a>(
        &'a self,
        mut st: S,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.iter().enumerate() {
                let s = x.format_element();
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        }
    }
}

// ============================================================================
// IMPL ASYNC: Iterator – COLLECTING versions
// ============================================================================
#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnImplAsync<
    F: Fn(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn iter_string_async_fn<'a>(self, f: &'a F) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringFnImplAsync<F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn<'a>(self, f: &'a F) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnImplAsyncSend<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > IteratorStringFnImplAsyncSend<F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutImplAsync<
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringFnMutImplAsync<F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutImplAsyncSend<
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > IteratorStringFnMutImplAsyncSend<F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateImplAsync<
    S,
    F: FnMut(&mut S, &str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn iter_string_with_state_async<'a>(
        self,
        st: S,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringWithStateImplAsync<S, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_async<'a>(
        self,
        mut st: S,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        }
    }
}

// ============================================================================
// IMPL ASYNC: Iterator – EXACT versions
// ============================================================================
#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnImplAsyncExact<
    F: Fn(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn iter_string_async_fn_exact<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter + ExactSizeIterator,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringFnImplAsyncExact<F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn_exact<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnImplAsyncSendExact<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn iter_string_async_fn_exact<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter + ExactSizeIterator + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > IteratorStringFnImplAsyncSendExact<F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn_exact<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutImplAsyncExact<
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn iter_string_async_fn_mut_exact<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter + ExactSizeIterator,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringFnMutImplAsyncExact<F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_async_fn_mut_exact<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutImplAsyncSendExact<
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn iter_string_async_fn_mut_exact<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter + ExactSizeIterator + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > IteratorStringFnMutImplAsyncSendExact<F, Fut> for I
where
    I::Item: FormatElement + Send,
{
    fn iter_string_async_fn_mut_exact<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(feature = "impl_async")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateImplAsyncExact<
    S,
    F: FnMut(&mut S, &str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn iter_string_with_state_async_exact<'a>(
        self,
        st: S,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a,
        Fut: 'a;
}

#[cfg(feature = "impl_async")]
impl<
        I: StableIter + ExactSizeIterator,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringWithStateImplAsyncExact<S, F, Fut> for I
where
    I::Item: FormatElement,
{
    fn iter_string_with_state_async_exact<'a>(
        self,
        mut st: S,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        S: 'a,
        Fut: 'a,
    {
        async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = x.format_element();
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        }
    }
}

// ============================================================================
// RAYON + DYN ASYNC / IMPL ASYNC (existing, unchanged)
// ============================================================================
#[cfg(all(feature = "rayon", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnAsync<'a, F, Fut> {
    fn par_iter_string_async_fn(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: core::future::Future<Output = String> + 'a,
        F: Fn(&str, usize, usize) -> Fut + Sync;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<'a, I, T, F, Fut> ParIteratorStringFnAsync<'a, F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
    T: FormatElement,
{
    fn par_iter_string_async_fn(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: core::future::Future<Output = String> + 'a,
        F: Fn(&str, usize, usize) -> Fut + Sync,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnAsyncSend<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<
        I: rayon::iter::ParallelIterator + Send,
        T: FormatElement + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > ParIteratorStringFnAsyncSend<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
        Fut: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnMutAsync<
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > ParIteratorStringFnMutAsync<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnMutAsyncSend<
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<
        I: rayon::iter::ParallelIterator + Send,
        T: FormatElement + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > ParIteratorStringFnMutAsyncSend<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
        Fut: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnPtrAsync {
    fn par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<I: rayon::iter::ParallelIterator, T: FormatElement> ParIteratorStringFnPtrAsync for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l));
            }
            r
        })
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnImplAsync<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String>,
>
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String>,
    > ParIteratorStringFnImplAsync<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnImplAsyncSend<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<
        I: rayon::iter::ParallelIterator + Send,
        T: FormatElement + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > ParIteratorStringFnImplAsyncSend<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnMutImplAsync<
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<
        I: rayon::iter::ParallelIterator,
        T: FormatElement,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > ParIteratorStringFnMutImplAsync<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnMutImplAsyncSend<
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<
        I: rayon::iter::ParallelIterator + Send,
        T: FormatElement + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > ParIteratorStringFnMutImplAsyncSend<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnPtrImplAsync {
    fn par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a;
}

#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<I: rayon::iter::ParallelIterator, T: FormatElement> ParIteratorStringFnPtrImplAsync for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
    {
        async move {
            let items: Vec<String> = self.map(|x| x.format_element()).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f(&s, i, l));
            }
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    use core::future::Future;

    // helper to block on a future without a real runtime (for testing)
    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    fn noop_raw_waker() -> core::task::RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(p: *const ()) -> core::task::RawWaker {
            core::task::RawWaker::new(p, &VTABLE)
        }
        static VTABLE: core::task::RawWakerVTable =
            core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
        core::task::RawWaker::new(core::ptr::null(), &VTABLE)
    }

    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    fn block_on<F: core::future::Future>(mut fut: F) -> F::Output {
        let mut fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };
        let waker = unsafe { core::task::Waker::from_raw(noop_raw_waker()) };
        let mut cx = core::task::Context::from_waker(&waker);
        loop {
            if let core::task::Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                return val;
            }
            core::hint::spin_loop();
        }
    }

    #[cfg(feature = "dyn_async")]
    fn block_on_dyn<'a, T>(fut: Box<dyn Future<Output = T> + 'a>) -> T {
        let mut pin_future = Box::into_pin(fut);
        let mut fut = pin_future.as_mut();
        let waker = unsafe { core::task::Waker::from_raw(noop_raw_waker()) };
        let mut cx = core::task::Context::from_waker(&waker);
        loop {
            if let core::task::Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                return val;
            }
            core::hint::spin_loop();
        }
    }

    // ─────────────────────────────────────────────────
    //  VecString (sync)
    // ─────────────────────────────────────────────────
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

    // ─────────────────────────────────────────────────
    //  IteratorString – collecting (works for any StableIter)
    // ─────────────────────────────────────────────────
    #[test]
    fn test_iterator_string_collecting() {
        let n = [1, 2, 3];
        assert_eq!(
            "[10, 20, 30]",
            IteratorString::iter_string(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iterator_string_collecting_non_exact() {
        let v = [1, 2, 3, 4, 5, 6];
        let filtered = v.iter().filter(|&x| *x % 2 == 0);
        // Filter is StableIter but not ExactSizeIterator, so only collecting version works
        assert_eq!(
            "[2, 4, 6]",
            IteratorString::iter_string(filtered, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iterator_string_collecting_empty() {
        let n: Vec<i32> = vec![];
        assert_eq!(
            "",
            IteratorString::iter_string(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iterator_string_collecting_single() {
        let n = [42];
        assert_eq!(
            "[420]",
            IteratorString::iter_string(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }

    // ─────────────────────────────────────────────────
    //  IteratorStringExact – no allocation
    // ─────────────────────────────────────────────────
    #[test]
    fn test_iterator_string_exact() {
        let n = [1, 2, 3];
        assert_eq!(
            "[10, 20, 30]",
            IteratorStringExact::iter_string_exact(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iterator_string_exact_empty() {
        let n: Vec<i32> = vec![];
        assert_eq!(
            "",
            IteratorStringExact::iter_string_exact(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iterator_string_exact_single() {
        let n = [42];
        assert_eq!(
            "[420]",
            IteratorStringExact::iter_string_exact(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }

    // ─────────────────────────────────────────────────
    //  IteratorStringFn / FnExact
    // ─────────────────────────────────────────────────
    #[test]
    fn test_iterator_string_fn_collecting() {
        let v = [1, 2, 3];
        let res = IteratorStringFn::iter_string_fn(v.iter(), |val, idx, total| {
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
    fn test_iterator_string_fn_exact() {
        let v = [1, 2, 3];
        let res = IteratorStringFnExact::iter_string_fn_exact(v.iter(), |val, idx, total| {
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

    // ─────────────────────────────────────────────────
    //  IteratorStringFnMut / FnMutExact
    // ─────────────────────────────────────────────────
    #[test]
    fn test_iterator_string_fn_mut_collecting() {
        let v = [10, 20, 30];
        let mut sum = 0;
        let res = IteratorStringFnMut::iter_string_fn_mut(v.iter(), |val, idx, total| {
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
    fn test_iterator_string_fn_mut_exact() {
        let v = [10, 20, 30];
        let mut sum = 0;
        let res =
            IteratorStringFnMutExact::iter_string_fn_mut_exact(v.iter(), |val, idx, total| {
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

    // ─────────────────────────────────────────────────
    //  Stateful (with state)
    // ─────────────────────────────────────────────────
    #[test]
    fn test_iterator_string_with_state_collecting() {
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
    fn test_iterator_string_with_state_exact() {
        let data = vec![1, 2, 3].into_iter();
        #[allow(unused_mut)]
        let mut sum = 0;
        let result = data.iter_string_with_state_exact(sum, |state, val, idx, total| {
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
    fn test_iterator_string_with_state_fn_exact() {
        let data = vec![1, 2, 3].into_iter();
        let multiplier = 10;
        let result = data.iter_string_with_state_fn_exact(&multiplier, |state, val, idx, total| {
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
    fn test_iterator_string_with_state_fn_ptr_exact() {
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
        let data = vec!["x", "y"].into_iter();
        let prefix = "##".to_string();
        assert_eq!(
            "[##x, ##y]",
            data.iter_string_with_state_fn_ptr_exact(&prefix, format_with_prefix)
        );
    }

    // ─────────────────────────────────────────────────
    //  RuleOwned / MutRuleOwned / RuleRef etc.
    // ─────────────────────────────────────────────────
    #[test]
    fn test_iterator_string_rule_owned_collecting() {
        let v = [10, 20, 30];
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
        assert_eq!("{10, 20, 30}", v.iter().iter_string_rule_owned(fmt));
    }

    #[test]
    fn test_iterator_string_rule_owned_exact() {
        let v = [10, 20, 30];
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
        assert_eq!("{10, 20, 30}", v.iter().iter_string_rule_owned_exact(fmt));
    }

    #[test]
    fn test_iterator_string_mut_rule_owned_collecting() {
        let v = [1, 2, 3];
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
        assert_eq!(
            "1, 2, 3 (total=6)",
            v.iter().iter_string_mut_rule_owned(fmt)
        );
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_iterator_string_mut_rule_owned_exact() {
        let v = [1, 2, 3];
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
        assert_eq!(
            "1, 2, 3 (total=6)",
            v.iter().iter_string_mut_rule_owned_exact(fmt)
        );
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_iterator_string_with_state_rule_owned_exact() {
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
        assert_eq!(
            "[10, 20, 30]",
            data.iter_string_with_state_rule_owned_exact(&multiplier, fmt)
        );
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule_owned_exact() {
        let data: Vec<&str> = vec!["hello", "world", "rust"];
        let positions: std::array::IntoIter<usize, 3> = [0usize, 1, 2].into_iter();
        let fmt = |pos: &mut std::array::IntoIter<usize, 3>, value: &str, index, length| {
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
        assert_eq!(
            "[hello, orld, st]",
            data.iter()
                .iter_string_with_state_mut_rule_owned_exact(positions, fmt)
        );
    }

    #[test]
    fn test_iterator_string_rule_ref_exact() {
        let v = [10, 20, 30];
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
        assert_eq!("{10, 20, 30}", v.iter().iter_string_rule_ref_exact(&fmt));
    }

    #[test]
    fn test_iterator_string_mut_rule_ref_exact() {
        let v = [1, 2, 3];
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
        assert_eq!(
            "1, 2, 3 (total=6)",
            v.iter().iter_string_mut_rule_ref_exact(&mut fmt)
        );
        assert_eq!(sum, 6);
    }

    // ─────────────────────────────────────────────────
    //  ExtendedDisplay
    // ─────────────────────────────────────────────────
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

    // ─────────────────────────────────────────────────
    //  Rayon sync (if feature = "rayon")
    // ─────────────────────────────────────────────────
    #[cfg(feature = "rayon")]
    #[test]
    fn test_rayon_par_iter_string() {
        use rayon::iter::IntoParallelIterator;
        let numbers = vec![1, 2, 3];
        assert_eq!(
            "[1, 2, 3]",
            ParIteratorString::par_iter_string(numbers.into_par_iter(), DEFAULT_FORMAT_RULE)
        );
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn test_rayon_par_iter_string_fn() {
        use rayon::prelude::*;
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
        assert_eq!(
            "{10, 20, 30}",
            ParIteratorStringFn::par_iter_string_fn(v.par_iter(), fmt)
        );
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn test_rayon_par_iter_methods() {
        use rayon::prelude::*;
        let v = vec![1, 2, 3];
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter().par_iter_string(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[2, 4, 6]",
            v.par_iter()
                .map(|x| x * 2)
                .par_iter_string(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.into_par_iter().par_iter_string(DEFAULT_FORMAT_RULE)
        );
    }

    // ─────────────────────────────────────────────────
    //  Dyn async tests
    // ─────────────────────────────────────────────────
    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_vec_string_fn_dyn_async() {
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        assert_eq!(
            "<1, 2, 3>",
            block_on_dyn(VecStringFnAsync::vec_string_async_fn(&v, &fmt))
        );
    }

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_iterator_string_fn_async_collecting() {
        let v = [10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        assert_eq!(
            "{10, 20, 30}",
            block_on_dyn(IteratorStringFnAsync::iter_string_async_fn(v.iter(), &fmt))
        );
    }

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_iterator_string_fn_async_exact() {
        let v = [10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        assert_eq!(
            "{10, 20, 30}",
            block_on_dyn(IteratorStringFnAsyncExact::iter_string_async_fn_exact(
                v.iter(),
                &fmt
            ))
        );
    }

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_iterator_string_fn_mut_async_collecting() {
        let v = ["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = |value: &str, _index: usize, _length: usize| {
            let value = value.to_string();
            counter += 1;
            let c = counter;
            async move { format!("[{}{}]", value, c) }
        };
        assert_eq!(
            "[a1][b2][c3]",
            block_on_dyn(IteratorStringFnMutAsync::iter_string_async_fn_mut(
                v.iter(),
                &mut fmt
            ))
        );
    }

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_iterator_string_fn_mut_async_exact() {
        let v = ["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = |value: &str, _index: usize, _length: usize| {
            let value = value.to_string();
            counter += 1;
            let c = counter;
            async move { format!("[{}{}]", value, c) }
        };
        assert_eq!(
            "[a1][b2][c3]",
            block_on_dyn(
                IteratorStringFnMutAsyncExact::iter_string_async_fn_mut_exact(v.iter(), &mut fmt)
            )
        );
    }

    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_iterator_string_with_state_async_exact() {
        let v = vec![1, 2, 3];
        #[allow(unused_mut)]
        let mut state = 10i32;
        let mut fmt = |s: &mut i32, val: &str, idx, len| {
            let val = val.to_string();
            *s += 1;
            let current = *s;
            async move {
                if len == 0 {
                    return String::new();
                }
                let is_last = idx == len - 1;
                if idx == 0 {
                    if is_last {
                        format!("[{}.{}]", val, current)
                    } else {
                        format!("[{}.{}", val, current)
                    }
                } else if is_last {
                    format!(", {}.{}]", val, current)
                } else {
                    format!(", {}.{}", val, current)
                }
            }
        };
        assert_eq!(
            "[1.11, 2.12, 3.13]",
            block_on_dyn(
                IteratorStringWithStateAsyncExact::iter_string_with_state_async_exact(
                    v.into_iter(),
                    state,
                    &mut fmt
                )
            )
        );
    }

    // ─────────────────────────────────────────────────
    //  Impl async tests
    // ─────────────────────────────────────────────────
    #[cfg(feature = "impl_async")]
    #[test]
    fn test_vec_string_fn_impl_async() {
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        assert_eq!(
            "<1, 2, 3>",
            block_on(VecStringFnImplAsync::vec_string_async_fn(&v, &fmt))
        );
    }

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_iterator_string_fn_impl_async_collecting() {
        let v = [10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        assert_eq!(
            "{10, 20, 30}",
            block_on(IteratorStringFnImplAsync::iter_string_async_fn(
                v.iter(),
                &fmt
            ))
        );
    }

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_iterator_string_fn_impl_async_exact() {
        let v = [10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        assert_eq!(
            "{10, 20, 30}",
            block_on(IteratorStringFnImplAsyncExact::iter_string_async_fn_exact(
                v.iter(),
                &fmt
            ))
        );
    }

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_iterator_string_fn_mut_impl_async_collecting() {
        let v = ["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = |value: &str, _index: usize, _length: usize| {
            let value = value.to_string();
            counter += 1;
            let c = counter;
            async move { format!("[{}{}]", value, c) }
        };
        assert_eq!(
            "[a1][b2][c3]",
            block_on(IteratorStringFnMutImplAsync::iter_string_async_fn_mut(
                v.iter(),
                &mut fmt
            ))
        );
    }

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_iterator_string_fn_mut_impl_async_exact() {
        let v = ["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = |value: &str, _index: usize, _length: usize| {
            let value = value.to_string();
            counter += 1;
            let c = counter;
            async move { format!("[{}{}]", value, c) }
        };
        assert_eq!(
            "[a1][b2][c3]",
            block_on(
                IteratorStringFnMutImplAsyncExact::iter_string_async_fn_mut_exact(
                    v.iter(),
                    &mut fmt
                )
            )
        );
    }

    #[cfg(feature = "impl_async")]
    #[test]
    fn test_iterator_string_with_state_impl_async_exact() {
        let v = vec![1, 2, 3];
        #[allow(unused_mut)]
        let mut state = String::from("val");
        let mut fmt = |s: &mut String, val: &str, idx, len| {
            s.push_str(val);
            let prefix = s.clone();
            async move {
                if len == 0 {
                    return String::new();
                }
                let is_last = idx == len - 1;
                if idx == 0 {
                    if is_last {
                        format!("[{}]", prefix)
                    } else {
                        format!("[{}", prefix)
                    }
                } else if is_last {
                    format!(", {}]", prefix)
                } else {
                    format!(", {}", prefix)
                }
            }
        };
        assert_eq!(
            "[val1, val12, val123]",
            block_on(
                IteratorStringWithStateImplAsyncExact::iter_string_with_state_async_exact(
                    v.into_iter(),
                    state,
                    &mut fmt
                )
            )
        );
    }

    // ─────────────────────────────────────────────────
    //  Rayon + async
    // ─────────────────────────────────────────────────
    #[cfg(all(feature = "rayon", feature = "dyn_async"))]
    #[test]
    fn test_rayon_par_iter_string_fn_dyn_async() {
        use rayon::iter::IntoParallelIterator;
        let v = vec![10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        let fut = ParIteratorStringFnAsync::par_iter_string_async_fn(v.into_par_iter(), &fmt);
        assert_eq!("{10, 20, 30}", block_on_dyn(fut));
    }

    #[cfg(all(feature = "rayon", feature = "impl_async"))]
    #[test]
    fn test_rayon_par_iter_string_fn_impl_async() {
        use rayon::iter::IntoParallelIterator;
        let v = vec![10, 20, 30];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move {
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
            }
        };
        assert_eq!(
            "{10, 20, 30}",
            block_on(ParIteratorStringFnImplAsync::par_iter_string_async_fn(
                v.into_par_iter(),
                &fmt
            ))
        );
    }

    // ─────────────────────────────────────────────────
    //  Misc corner cases
    // ─────────────────────────────────────────────────
    #[test]
    fn test_custom_format_rule_no_brackets() {
        let v = vec![1, 2, 3];
        let res = VecString::vec_string(&v, |val, idx, total| {
            if total == 0 {
                return String::new();
            }
            if idx == total - 1 {
                val.to_string()
            } else {
                format!("{}, ", val)
            }
        });
        assert_eq!(res, "1, 2, 3");
    }

    #[test]
    fn test_exact_size_vs_collecting_consistency() {
        let data = [5, 10, 15];
        let fmt = |val: &str, _i, _t| format!("({})", val);
        let col_res = IteratorStringFn::iter_string_fn(data.iter(), fmt);
        let exact_res = IteratorStringFnExact::iter_string_fn_exact(data.iter(), fmt);
        assert_eq!(col_res, exact_res);
    }
}