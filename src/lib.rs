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
// StableIter
// ============================================================================
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
pub trait ExtendedDisplay {}
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
impl<I> ExtendedDisplay for I
where
    I: StableIter,
    I::Item: core::fmt::Display,
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
pub trait FormatRuleNoState<'a> {
    fn format(&'a self, value: &str, index: usize, length: usize) -> String;
}
impl<'a, F: Fn(&str, usize, usize) -> String> FormatRuleNoState<'a> for F {
    fn format(&'a self, v: &str, i: usize, l: usize) -> String {
        (self)(v, i, l)
    }
}
pub trait FormatRuleNoStateOwned {
    fn format(self, value: &str, index: usize, length: usize) -> String;
}
impl<F: Fn(&str, usize, usize) -> String> FormatRuleNoStateOwned for F {
    fn format(self, v: &str, i: usize, l: usize) -> String {
        (self)(v, i, l)
    }
}
pub trait FormatRuleMutNoState {
    fn format(&mut self, value: &str, index: usize, length: usize) -> String;
}
impl<F: FnMut(&str, usize, usize) -> String> FormatRuleMutNoState for F {
    fn format(&mut self, v: &str, i: usize, l: usize) -> String {
        (self)(v, i, l)
    }
}
pub trait FormatRule<S> {
    fn format(&self, state: &S, value: &str, index: usize, length: usize) -> String;
}
impl<S, F: Fn(&S, &str, usize, usize) -> String> FormatRule<S> for F {
    fn format(&self, s: &S, v: &str, i: usize, l: usize) -> String {
        (self)(s, v, i, l)
    }
}
pub trait FormatRuleMut<S> {
    fn format(&mut self, state: &mut S, value: &str, index: usize, length: usize) -> String;
}
impl<S, F: FnMut(&mut S, &str, usize, usize) -> String> FormatRuleMut<S> for F {
    fn format(&mut self, s: &mut S, v: &str, i: usize, l: usize) -> String {
        (self)(s, v, i, l)
    }
}

// ============================================================================
// SYNC VecString* / IteratorString*
// ============================================================================
pub trait VecString {
    fn vec_string(&self, format_rule: FormatRuleFn) -> String;
}
pub trait VecStringFn<F: Fn(&str, usize, usize) -> String> {
    fn vec_string_fn(&self, format_rule: F) -> String;
}
pub trait VecStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn vec_string_fn_mut(&self, format_rule: F) -> String;
}
impl<T: core::fmt::Display> VecString for Vec<T> {
    fn vec_string(&self, f: FormatRuleFn) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&format!("{}", x), i, l));
        }
        s
    }
}
impl<T: core::fmt::Display, F: Fn(&str, usize, usize) -> String> VecStringFn<F> for Vec<T> {
    fn vec_string_fn(&self, f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&format!("{}", x), i, l));
        }
        s
    }
}
impl<T: core::fmt::Display, F: FnMut(&str, usize, usize) -> String> VecStringFnMut<F> for Vec<T> {
    fn vec_string_fn_mut(&self, mut f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&format!("{}", x), i, l));
        }
        s
    }
}
pub trait IteratorString {
    fn iter_string(self, format_rule: FormatRuleFn) -> String;
}
pub trait IteratorStringFn<F: Fn(&str, usize, usize) -> String> {
    fn iter_string_fn(self, format_rule: F) -> String;
}
pub trait IteratorStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn iter_string_fn_mut(self, format_rule: F) -> String;
}
impl<I: StableIter> IteratorString for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
    I::Item: core::fmt::Display,
{
    fn iter_string_fn(self, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
    I::Item: core::fmt::Display,
{
    fn iter_string_fn_mut(self, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}
pub trait VecStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn vec_string_with_state(&self, st: S, f: F) -> String;
}
pub trait IteratorStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn iter_string_with_state(self, st: S, f: F) -> String;
}
impl<T: core::fmt::Display, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    VecStringWithState<S, F> for Vec<T>
{
    fn vec_string_with_state(&self, mut st: S, mut f: F) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}
impl<I: StableIter, S, F: FnMut(&mut S, &str, usize, usize) -> String> IteratorStringWithState<S, F>
    for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state(self, mut st: S, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}
pub trait VecStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn vec_string_with_state_fn(&self, st: &S, f: F) -> String;
}
pub trait IteratorStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn iter_string_with_state_fn(self, st: &S, f: F) -> String;
}
impl<T: core::fmt::Display, S, F: Fn(&S, &str, usize, usize) -> String> VecStringWithStateFn<S, F>
    for Vec<T>
{
    fn vec_string_with_state_fn(&self, st: &S, f: F) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}
impl<I: StableIter, S, F: Fn(&S, &str, usize, usize) -> String> IteratorStringWithStateFn<S, F>
    for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_fn(self, st: &S, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}
pub trait VecStringWithStateFnPtr<S> {
    fn vec_string_with_state_fn_ptr(
        &self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}
pub trait IteratorStringWithStateFnPtr<S> {
    fn iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}
impl<T: core::fmt::Display, S> VecStringWithStateFnPtr<S> for Vec<T> {
    fn vec_string_with_state_fn_ptr(
        &self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}
impl<I: StableIter, S> IteratorStringWithStateFnPtr<S> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// SYNC RuleOwned / MutRuleOwned / RuleRef / MutRuleRef
// ============================================================================
pub trait VecStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn vec_string_rule_owned(self, rule: R) -> String;
}
impl<T: core::fmt::Display, R: FormatRuleNoStateOwned + Clone> VecStringRuleOwned<R> for Vec<T> {
    fn vec_string_rule_owned(self, rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.clone().format(&format!("{}", x), i, l));
        }
        s
    }
}
pub trait VecStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_owned(&self, rule: R) -> String;
}
impl<T: core::fmt::Display, R: FormatRuleMutNoState> VecStringMutRuleOwned<R> for Vec<T> {
    fn vec_string_mut_rule_owned(&self, mut rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&format!("{}", x), i, l));
        }
        s
    }
}
pub trait IteratorStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn iter_string_rule_owned(self, rule: R) -> String;
}
impl<I: StableIter, R: FormatRuleNoStateOwned + Clone> IteratorStringRuleOwned<R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_rule_owned(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.clone().format(&s, i, l));
        }
        r
    }
}
pub trait IteratorStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_owned(self, rule: R) -> String;
}
impl<I: StableIter, R: FormatRuleMutNoState> IteratorStringMutRuleOwned<R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_mut_rule_owned(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}
pub trait VecStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_owned(&self, st: &S, rule: R) -> String;
}
impl<T: core::fmt::Display, S, R: FormatRule<S>> VecStringWithStateRuleOwned<S, R> for Vec<T> {
    fn vec_string_with_state_rule_owned(&self, st: &S, rule: R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}
pub trait IteratorStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String;
}
impl<I: StableIter, S, R: FormatRule<S>> IteratorStringWithStateRuleOwned<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}
pub trait VecStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_owned(&self, st: S, rule: R) -> String;
}
impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleOwned<S, R>
    for Vec<T>
{
    fn vec_string_with_state_mut_rule_owned(&self, mut st: S, mut rule: R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}
pub trait IteratorStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_owned(self, st: S, rule: R) -> String;
}
impl<I: StableIter, S, R: FormatRuleMut<S>> IteratorStringWithStateMutRuleOwned<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_mut_rule_owned(self, mut st: S, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}
pub trait VecStringRuleRef<'a, R: FormatRuleNoState<'a>> {
    fn vec_string_rule_ref(&self, rule: &'a R) -> String;
}
impl<'a, T: core::fmt::Display, R: FormatRuleNoState<'a>> VecStringRuleRef<'a, R> for Vec<T> {
    fn vec_string_rule_ref(&self, rule: &'a R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&format!("{}", x), i, l));
        }
        s
    }
}
pub trait VecStringMutRuleRef<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String;
}
impl<T: core::fmt::Display, R: FormatRuleMutNoState> VecStringMutRuleRef<R> for Vec<T> {
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&format!("{}", x), i, l));
        }
        s
    }
}
pub trait IteratorStringRuleRef<'a, R: FormatRuleNoState<'a>> {
    fn iter_string_rule_ref(self, rule: &'a R) -> String;
}
impl<'a, I: StableIter, R: FormatRuleNoState<'a>> IteratorStringRuleRef<'a, R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_rule_ref(self, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}
pub trait IteratorStringMutRuleRef<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_ref(self, rule: &mut R) -> String;
}
impl<I: StableIter, R: FormatRuleMutNoState> IteratorStringMutRuleRef<R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_mut_rule_ref(self, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}
pub trait VecStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_ref(&self, st: &S, rule: &R) -> String;
}
impl<T: core::fmt::Display, S, R: FormatRule<S>> VecStringWithStateRuleRef<S, R> for Vec<T> {
    fn vec_string_with_state_rule_ref(&self, st: &S, rule: &R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}
pub trait IteratorStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String;
}
impl<I: StableIter, S, R: FormatRule<S>> IteratorStringWithStateRuleRef<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}
pub trait VecStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_ref(&self, st: S, rule: &mut R) -> String;
}
impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleRef<S, R> for Vec<T> {
    fn vec_string_with_state_mut_rule_ref(&self, mut st: S, rule: &mut R) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = format!("{}", x);
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}
pub trait IteratorStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_ref(self, st: S, rule: &mut R) -> String;
}
impl<I: StableIter, S, R: FormatRuleMut<S>> IteratorStringWithStateMutRuleRef<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_mut_rule_ref(self, mut st: S, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// RAYON SYNC
// ============================================================================
#[cfg(feature = "rayon")]
pub trait ParIteratorString {
    fn par_iter_string(self, f: FormatRuleFn) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display> ParIteratorString for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringFn<F: Fn(&str, usize, usize) -> String> {
    fn par_iter_string_fn(self, f: F) -> String;
}
#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> String + Sync,
    > ParIteratorStringFn<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn(self, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn par_iter_string_fn_mut(self, f: F) -> String;
}
#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> String,
    > ParIteratorStringFnMut<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn_mut(self, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}
#[cfg(feature = "rayon")]
pub trait ParIteratorStringFnPtr {
    fn par_iter_string_fn_ptr(self, f: FormatRuleFn) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display> ParIteratorStringFnPtr for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn_ptr(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn par_iter_string_with_state(self, st: S, f: F) -> String;
}
#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: core::fmt::Display,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> String,
    > ParIteratorStringWithState<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state(self, mut st: S, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}
#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn par_iter_string_with_state_fn(self, st: &S, f: F) -> String;
}
#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: core::fmt::Display,
        S: Sync,
        F: Fn(&S, &str, usize, usize) -> String + Sync,
    > ParIteratorStringWithStateFn<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_fn(self, st: &S, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringWithStateFnPtr<S> {
    fn par_iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display, S: Sync>
    ParIteratorStringWithStateFnPtr<S> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn par_iter_string_rule_owned(self, rule: R) -> String;
}
#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: core::fmt::Display,
        R: FormatRuleNoStateOwned + Clone + Sync,
    > ParIteratorStringRuleOwned<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_rule_owned(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn par_iter_string_mut_rule_owned(self, rule: R) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display, R: FormatRuleMutNoState>
    ParIteratorStringMutRuleOwned<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_mut_rule_owned(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}
#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn par_iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display, S: Sync, R: FormatRule<S> + Sync>
    ParIteratorStringWithStateRuleOwned<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn par_iter_string_with_state_mut_rule_owned(self, st: S, rule: R) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display, S, R: FormatRuleMut<S>>
    ParIteratorStringWithStateMutRuleOwned<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_mut_rule_owned(self, mut st: S, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}
#[cfg(feature = "rayon")]
pub trait ParIteratorStringRuleRef<'a, R: FormatRuleNoState<'a>> {
    fn par_iter_string_rule_ref(self, rule: &'a R) -> String;
}
#[cfg(feature = "rayon")]
impl<
        'a,
        I: rayon::iter::ParallelIterator,
        T: core::fmt::Display,
        R: FormatRuleNoState<'a> + Sync,
    > ParIteratorStringRuleRef<'a, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_rule_ref(self, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringMutRuleRef<R: FormatRuleMutNoState> {
    fn par_iter_string_mut_rule_ref(self, rule: &mut R) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display, R: FormatRuleMutNoState>
    ParIteratorStringMutRuleRef<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_mut_rule_ref(self, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}
#[cfg(feature = "rayon")]
pub trait ParIteratorStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn par_iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display, S: Sync, R: FormatRule<S> + Sync>
    ParIteratorStringWithStateRuleRef<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn par_iter_string_with_state_mut_rule_ref(self, st: S, rule: &mut R) -> String;
}
#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display, S, R: FormatRuleMut<S>>
    ParIteratorStringWithStateMutRuleRef<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_mut_rule_ref(self, mut st: S, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
// DYN ASYNC: Vec/Iterator (с ExactSizeIterator — без collect)
// ============================================================================
#[cfg(feature = "dyn_async")]
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
        T: core::fmt::Display,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        T: core::fmt::Display + Sync,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        T: core::fmt::Display,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        T: core::fmt::Display + Sync,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        T: core::fmt::Display,
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
                let s = format!("{}", x);
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        })
    }
}

// Iterator async — ExactSizeIterator (без collect)
#[cfg(feature = "dyn_async")]
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
        I: StableIter + ExactSizeIterator,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringFnAsync<'a, F, Fut> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_async_fn(self, f: &'a F) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
    {
        Box::new(async move {
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        I: StableIter + ExactSizeIterator + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > IteratorStringFnAsyncSend<'a, F, Fut> for I
where
    I::Item: core::fmt::Display + Send,
{
    fn iter_string_async_fn(
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        I: StableIter + ExactSizeIterator,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringFnMutAsync<'a, F, Fut> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_async_fn_mut(
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        I: StableIter + ExactSizeIterator + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > IteratorStringFnMutAsyncSend<'a, F, Fut> for I
where
    I::Item: core::fmt::Display + Send,
{
    fn iter_string_async_fn_mut(
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        })
    }
}
#[cfg(feature = "dyn_async")]
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
        I: StableIter + ExactSizeIterator,
        S: 'a,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > IteratorStringWithStateAsync<'a, S, F, Fut> for I
where
    I::Item: core::fmt::Display,
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
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = format!("{}", x);
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
pub trait FormatRuleNoStateImplAsyncSend<Fut: core::future::Future<Output = String> + Send> {
    fn format(
        &self,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}
#[cfg(feature = "impl_async")]
impl<F: Fn(&str, usize, usize) -> Fut, Fut: core::future::Future<Output = String> + Send>
    FormatRuleNoStateImplAsyncSend<Fut> for F
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
pub trait FormatRuleNoStateOwnedImplAsyncSend<Fut: core::future::Future<Output = String> + Send> {
    fn format(
        self,
        value: String,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}
#[cfg(feature = "impl_async")]
impl<F: FnOnce(String, usize, usize) -> Fut, Fut: core::future::Future<Output = String> + Send>
    FormatRuleNoStateOwnedImplAsyncSend<Fut> for F
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
pub trait FormatRuleMutNoStateImplAsyncSend<Fut: core::future::Future<Output = String> + Send> {
    fn format(
        &mut self,
        value: &str,
        index: usize,
        length: usize,
    ) -> impl core::future::Future<Output = String> + Send;
}
#[cfg(feature = "impl_async")]
impl<F: FnMut(&str, usize, usize) -> Fut, Fut: core::future::Future<Output = String> + Send>
    FormatRuleMutNoStateImplAsyncSend<Fut> for F
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
        S,
        F: Fn(&S, &str, usize, usize) -> Fut,
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
        S,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
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
// IMPL ASYNC: Vec/Iterator (с ExactSizeIterator — без collect)
// ============================================================================
#[cfg(feature = "impl_async")]
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
        T: core::fmt::Display,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        T: core::fmt::Display + Sync,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        T: core::fmt::Display,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        T: core::fmt::Display + Sync,
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        T: core::fmt::Display,
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
                let s = format!("{}", x);
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        }
    }
}

// Iterator impl_async — ExactSizeIterator (без collect)
#[cfg(feature = "impl_async")]
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
        I: StableIter + ExactSizeIterator,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringFnImplAsync<F, Fut> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_async_fn<'a>(self, f: &'a F) -> impl core::future::Future<Output = String> + 'a
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
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        I: StableIter + ExactSizeIterator + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > IteratorStringFnImplAsyncSend<F, Fut> for I
where
    I::Item: core::fmt::Display + Send,
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
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        I: StableIter + ExactSizeIterator,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringFnMutImplAsync<F, Fut> for I
where
    I::Item: core::fmt::Display,
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
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        I: StableIter + ExactSizeIterator + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > IteratorStringFnMutImplAsyncSend<F, Fut> for I
where
    I::Item: core::fmt::Display + Send,
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
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = format!("{}", x);
                r.push_str(&f(&s, i, l).await);
            }
            r
        }
    }
}
#[cfg(feature = "impl_async")]
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
        I: StableIter + ExactSizeIterator,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > IteratorStringWithStateImplAsync<S, F, Fut> for I
where
    I::Item: core::fmt::Display,
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
            let l = self.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, x) in self.enumerate() {
                let s = format!("{}", x);
                r.push_str(&f(&mut st, &s, i, l).await);
            }
            r
        }
    }
}

// ============================================================================
// RAYON + DYN ASYNC
// ============================================================================
#[cfg(all(feature = "rayon", feature = "dyn_async"))]
pub trait ParIteratorStringFnAsync<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String>,
>
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: 'a;
}
#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<
        I: rayon::iter::ParallelIterator,
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String>,
    > ParIteratorStringFnAsync<F, Fut> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: 'a,
    {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
        T: core::fmt::Display + Send,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
        T: core::fmt::Display,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
        T: core::fmt::Display + Send,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringFnPtrAsync {
    fn par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}
#[cfg(all(feature = "rayon", feature = "dyn_async"))]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display> ParIteratorStringFnPtrAsync for I
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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

// ============================================================================
// RAYON + IMPL ASYNC
// ============================================================================
#[cfg(all(feature = "rayon", feature = "impl_async"))]
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
        T: core::fmt::Display,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
        T: core::fmt::Display + Send,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
        T: core::fmt::Display,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
        T: core::fmt::Display + Send,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
pub trait ParIteratorStringFnPtrImplAsync {
    fn par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a;
}
#[cfg(all(feature = "rayon", feature = "impl_async"))]
impl<I: rayon::iter::ParallelIterator, T: core::fmt::Display> ParIteratorStringFnPtrImplAsync for I
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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

// ============================================================================
// ТЕСТЫ
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    use core::future::Future;
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
        let n = vec![1, 2, 3];
        assert_eq!(
            "[10, 20, 30]",
            IteratorString::iter_string(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }
    #[test]
    fn test_iterator_string_empty() {
        let n: Vec<i32> = vec![];
        assert_eq!(
            "",
            IteratorString::iter_string(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }
    #[test]
    fn test_iterator_string_single() {
        let n = vec![42];
        assert_eq!(
            "[420]",
            IteratorString::iter_string(n.iter().map(|x| x * 10), DEFAULT_FORMAT_RULE)
        );
    }
    #[test]
    fn test_vec_string_fn() {
        let v = vec!["a", "bb", "ccc"];
        let res = VecStringFn::vec_string_fn(&v, |val, idx, total| {
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
        let res = VecStringFn::vec_string_fn(&v, |val, idx, total| {
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
        let res = VecStringFnMut::vec_string_fn_mut(&v, |val, _idx, _total| {
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
        let res = VecStringFnMut::vec_string_fn_mut(&v, |val, _idx, _total| {
            counter += 1;
            format!("{}{}", val, counter)
        });
        assert_eq!(res, "");
        assert_eq!(counter, 0);
    }
    #[test]
    fn test_iterator_string_fn() {
        let v = vec![1, 2, 3];
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
    fn test_iterator_string_fn_mut() {
        let v = vec![10, 20, 30];
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
        assert_eq!(
            "[>>a, >>b, >>c]",
            data.vec_string_with_state_fn_ptr(&prefix, format_with_prefix)
        );
    }
    #[test]
    fn test_iterator_string_with_state_fn_ptr() {
        let data = vec!["x", "y"].into_iter();
        let prefix = "##".to_string();
        assert_eq!(
            "[##x, ##y]",
            data.iter_string_with_state_fn_ptr(&prefix, format_with_prefix)
        );
    }
    #[test]
    fn test_vec_string_with_state_fn_ptr_empty() {
        let data: Vec<&str> = vec![];
        let prefix = ">>".to_string();
        assert_eq!(
            "",
            data.vec_string_with_state_fn_ptr(&prefix, format_with_prefix)
        );
    }
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
        assert_eq!("<1, 2, 3>", v.vec_string_rule_owned(fmt));
    }
    #[test]
    fn test_vec_string_mut_rule_owned() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0;
        let fmt = |value: &str, _index: usize, _length: usize| {
            counter += 1;
            format!("[{}{}]", value, counter)
        };
        assert_eq!("[a1][b2][c3]", v.vec_string_mut_rule_owned(fmt));
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
        assert_eq!("{10, 20, 30}", v.iter().iter_string_rule_owned(fmt));
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
        assert_eq!(
            "1, 2, 3 (total=6)",
            v.iter().iter_string_mut_rule_owned(fmt)
        );
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
        assert_eq!(
            "[>>hello, >>world]",
            data.vec_string_with_state_rule_owned(&prefix, fmt)
        );
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
        assert_eq!(
            "[10, 20, 30]",
            data.iter_string_with_state_rule_owned(&multiplier, fmt)
        );
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
        assert_eq!(
            "(sum=1: 1, sum=3: 2, sum=6: 3)",
            data.vec_string_with_state_mut_rule_owned(sum, fmt)
        );
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
        assert_eq!(
            "[hello, orld, st]",
            data.iter()
                .iter_string_with_state_mut_rule_owned(positions, fmt)
        );
    }
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
        assert_eq!("<1, 2, 3>", v.vec_string_rule_ref(&fmt));
    }
    #[test]
    fn test_vec_string_mut_rule_ref() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0;
        let mut fmt = |value: &str, _index: usize, _length: usize| {
            counter += 1;
            format!("[{}{}]", value, counter)
        };
        assert_eq!("[a1][b2][c3]", v.vec_string_mut_rule_ref(&mut fmt));
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
        assert_eq!("{10, 20, 30}", v.iter().iter_string_rule_ref(&fmt));
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
        assert_eq!(
            "1, 2, 3 (total=6)",
            v.iter().iter_string_mut_rule_ref(&mut fmt)
        );
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
        assert_eq!(
            "[>>hello, >>world]",
            data.vec_string_with_state_rule_ref(&prefix, &fmt)
        );
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
        assert_eq!(
            "[10, 20, 30]",
            data.iter_string_with_state_rule_ref(&multiplier, &fmt)
        );
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
        assert_eq!(
            "(sum=1: 1, sum=3: 2, sum=6: 3)",
            data.vec_string_with_state_mut_rule_ref(sum, &mut fmt)
        );
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
        assert_eq!(
            "[hello, orld, st]",
            data.iter()
                .iter_string_with_state_mut_rule_ref(positions, &mut fmt)
        );
    }
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
    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_vec_string_fn_dyn_async() {
        let v = vec![1, 2, 3];
        let fmt = |value: &str,
                   index: usize,
                   length: usize|
         -> Box<dyn core::future::Future<Output = String> + '_> {
            let value = value.to_string();
            Box::new(async move {
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
            })
        };
        assert_eq!(
            "<1, 2, 3>",
            block_on_dyn(VecStringFnAsync::vec_string_async_fn(&v, &fmt))
        );
    }
    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_iterator_string_fn_dyn_async() {
        let v = vec![10, 20, 30];
        let fmt = |value: &str,
                   index: usize,
                   length: usize|
         -> Box<dyn core::future::Future<Output = String> + '_> {
            let value = value.to_string();
            Box::new(async move {
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
            })
        };
        assert_eq!(
            "{10, 20, 30}",
            block_on_dyn(IteratorStringFnAsync::iter_string_async_fn(v.iter(), &fmt))
        );
    }
    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_vec_string_fn_mut_dyn_async() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = |value: &str,
                       _index: usize,
                       _length: usize|
         -> Box<dyn core::future::Future<Output = String> + '_> {
            let value = value.to_string();
            counter += 1;
            let c = counter;
            Box::new(async move { format!("[{}{}]", value, c) })
        };
        assert_eq!(
            "[a1][b2][c3]",
            block_on_dyn(VecStringFnMutAsync::vec_string_async_fn_mut(&v, &mut fmt))
        );
    }
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
    fn test_iterator_string_fn_impl_async() {
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
            block_on(IteratorStringFnImplAsync::iter_string_async_fn(
                v.iter(),
                &fmt
            ))
        );
    }
    #[cfg(feature = "impl_async")]
    #[test]
    fn test_vec_string_fn_mut_impl_async() {
        let v = vec!["a", "b", "c"];
        let mut counter = 0usize;
        let mut fmt = |value: &str, _index: usize, _length: usize| {
            let value = value.to_string();
            counter += 1;
            let c = counter;
            async move { format!("[{}{}]", value, c) }
        };
        assert_eq!(
            "[a1][b2][c3]",
            block_on(VecStringFnMutImplAsync::vec_string_async_fn_mut(
                &v, &mut fmt
            ))
        );
    }
    #[cfg(all(feature = "rayon", feature = "dyn_async"))]
    #[test]
    fn test_rayon_par_iter_string_fn_dyn_async() {
        use rayon::iter::IntoParallelIterator;
        let v = vec![10, 20, 30];
        let fmt = |value: &str,
                   index: usize,
                   length: usize|
         -> Box<dyn core::future::Future<Output = String> + '_> {
            let value = value.to_string();
            Box::new(async move {
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
            })
        };
        assert_eq!(
            "{10, 20, 30}",
            block_on_dyn(ParIteratorStringFnAsync::par_iter_string_async_fn(
                v.into_par_iter(),
                &fmt
            ))
        );
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
}
