#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
#[cfg(any(feature = "dyn_async", feature = "impl_async"))]
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "orx_parallel")]
use orx_parallel::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

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
// StableIter impls for itertools
// ============================================================================
#[cfg(feature = "itertools")]
impl<I, J> StableIter for itertools::Interleave<I, J>
where
    I: StableIter,
    J: StableIter<Item = I::Item>,
{
}
#[cfg(feature = "itertools")]
impl<I, J> StableIter for itertools::InterleaveShortest<I, J>
where
    I: StableIter,
    J: StableIter<Item = I::Item>,
{
}
#[cfg(feature = "itertools")]
impl<I, J> StableIter for itertools::Product<I, J>
where
    I: StableIter,
    I::Item: Clone,
    J: StableIter + Clone,
{
}
#[cfg(feature = "itertools")]
impl<I, F, B> StableIter for itertools::Batching<I, F>
where
    I: StableIter,
    F: FnMut(&mut I) -> Option<B>,
{
}
#[cfg(feature = "itertools")]
impl<I, T> StableIter for itertools::WhileSome<I> where I: StableIter<Item = Option<T>> {}
#[cfg(feature = "itertools")]
impl<I, F, T, E> StableIter for itertools::FilterOk<I, F>
where
    I: StableIter<Item = Result<T, E>>,
    F: FnMut(&T) -> bool,
{
}
#[cfg(feature = "itertools")]
impl<I, F, T, E, U> StableIter for itertools::FilterMapOk<I, F>
where
    I: StableIter<Item = Result<T, E>>,
    F: FnMut(T) -> Option<U>,
{
}
#[cfg(feature = "itertools")]
impl<I, F> StableIter for itertools::Positions<I, F>
where
    I: StableIter,
    F: FnMut(I::Item) -> bool,
{
}
#[cfg(feature = "itertools")]
impl<I, F> StableIter for itertools::Update<I, F>
where
    I: StableIter,
    F: FnMut(&mut I::Item),
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::Combinations<I>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itertools")]
impl<I, const K: usize> StableIter for itertools::ArrayCombinations<I, K>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::CombinationsWithReplacement<I>
where
    I: StableIter,
    I::Item: Clone + Ord,
{
}
#[cfg(feature = "itertools")]
impl<I, F> StableIter for itertools::PadUsing<I, F>
where
    I: StableIter,
    F: FnMut(usize) -> I::Item,
{
}
#[cfg(feature = "itertools")]
impl<'a, I, F> StableIter for itertools::PeekingTakeWhile<'a, I, F>
where
    I: itertools::PeekingNext,
    F: FnMut(&I::Item) -> bool,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::Permutations<I>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::Powerset<I>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itertools")]
impl<'a, I, T, E> StableIter for itertools::ProcessResults<'a, I, E> where
    I: StableIter<Item = Result<T, E>>
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::Tee<I>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::Unique<I>
where
    I: StableIter,
    I::Item: Clone + Eq + core::hash::Hash,
{
}
#[cfg(feature = "itertools")]
impl<I, V, F> StableIter for itertools::UniqueBy<I, V, F>
where
    I: StableIter,
    V: Eq + core::hash::Hash,
    F: FnMut(&I::Item) -> V,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::WithPosition<I> where I: StableIter {}
#[cfg(feature = "itertools")]
impl<I, J> StableIter for itertools::ZipEq<I, J>
where
    I: StableIter,
    J: StableIter,
{
}
#[cfg(feature = "itertools")]
impl<T, U> StableIter for itertools::ZipLongest<T, U>
where
    T: StableIter,
    U: StableIter,
{
}
#[cfg(feature = "itertools")]
impl<I, R> StableIter for itertools::MapInto<I, R>
where
    I: StableIter,
    R: From<I::Item>,
{
}
#[cfg(feature = "itertools")]
impl<I, T, E> StableIter for itertools::FlattenOk<I, T, E>
where
    I: StableIter<Item = Result<T, E>>,
    T: IntoIterator,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::MultiProduct<I>
where
    I: StableIter + Clone,
    I::Item: Clone,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::structs::PutBack<I> where I: StableIter {}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::PutBackN<I> where I: StableIter {}
#[cfg(feature = "itertools")]
impl<I, F> StableIter for itertools::structs::TakeWhileInclusive<I, F>
where
    I: StableIter,
    F: FnMut(&I::Item) -> bool,
{
}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::structs::ExactlyOneError<I> where I: StableIter {}
#[cfg(feature = "itertools")]
impl<A: Clone> StableIter for itertools::structs::RepeatN<A> {}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::structs::MultiPeek<I> where I: StableIter {}
#[cfg(feature = "itertools")]
impl<I> StableIter for itertools::structs::PeekNth<I> where I: StableIter {}

// ============================================================================
// StableIter impls for itermore
// ============================================================================
#[cfg(feature = "itermore")]
impl<I, const N: usize> StableIter for itermore::ArrayChunks<I, N> where I: StableIter {}
#[cfg(feature = "itermore")]
impl<I, const N: usize> StableIter for itermore::ArrayWindows<I, N>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itermore")]
impl<I, const K: usize> StableIter for itermore::ArrayCombinations<I, K>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itermore")]
impl<I, const K: usize> StableIter for itermore::ArrayCombinationsWithReps<I, K>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itermore")]
impl<I, J> StableIter for itermore::CartesianProduct<I, J>
where
    I: StableIter,
    I::Item: Clone,
    J: StableIter + Clone,
{
}
#[cfg(feature = "itermore")]
impl<I, const N: usize> StableIter for itermore::CircularArrayWindows<I, N>
where
    I: StableIter + Clone,
    I::Item: Clone,
{
}
#[cfg(feature = "itermore")]
impl<I> StableIter for itermore::Combinations<I>
where
    I: StableIter,
    I::Item: Clone,
{
}
#[cfg(feature = "itermore")]
impl<I> StableIter for itermore::CombinationsWithReps<I>
where
    I: StableIter,
    I::Item: Clone,
{
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ExtendedDisplay {}

impl<T: core::fmt::Display> ExtendedDisplay for Vec<T> {}

impl<T: core::fmt::Display> ExtendedDisplay for [T] {}

impl<I> ExtendedDisplay for I
where
    I: StableIter,
    I::Item: core::fmt::Display,
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
// SYNC TRAITS FOR RULES
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait FormatRuleNoState {
    fn format(&self, value: &str, index: usize, length: usize) -> String;
}
impl<F: Fn(&str, usize, usize) -> String> FormatRuleNoState for F {
    fn format(&self, v: &str, i: usize, l: usize) -> String {
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

impl<T: core::fmt::Display> VecString for [T] {
    fn vec_string(&self, f: FormatRuleFn) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&format!("{}", x), i, l));
        }
        s
    }
}

impl<T: core::fmt::Display, F: Fn(&str, usize, usize) -> String> VecStringFn<F> for [T] {
    fn vec_string_fn(&self, f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&format!("{}", x), i, l));
        }
        s
    }
}

impl<T: core::fmt::Display, F: FnMut(&str, usize, usize) -> String> VecStringFnMut<F> for [T] {
    fn vec_string_fn_mut(&self, mut f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&format!("{}", x), i, l));
        }
        s
    }
}

// Vec<T> impls for base traits (delegate to [T] via Deref)
impl<T: core::fmt::Display> VecString for Vec<T> {
    fn vec_string(&self, f: FormatRuleFn) -> String {
        self.as_slice().vec_string(f)
    }
}

impl<T: core::fmt::Display, F: Fn(&str, usize, usize) -> String> VecStringFn<F> for Vec<T> {
    fn vec_string_fn(&self, f: F) -> String {
        self.as_slice().vec_string_fn(f)
    }
}

impl<T: core::fmt::Display, F: FnMut(&str, usize, usize) -> String> VecStringFnMut<F> for Vec<T> {
    fn vec_string_fn_mut(&self, f: F) -> String {
        self.as_slice().vec_string_fn_mut(f)
    }
}

// ============================================================================
// NESTED TRAITS - recursive formatting for Vec<Vec<T>> etc.
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringNested {
    fn vec_string_nested(&self, inner_rule: FormatRuleFn, format_rule: FormatRuleFn) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnNested<F: Fn(&str, usize, usize) -> String> {
    fn vec_string_fn_nested(&self, inner_rule: FormatRuleFn, format_rule: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringFnMutNested<F: FnMut(&str, usize, usize) -> String> {
    fn vec_string_fn_mut_nested(&self, inner_rule: FormatRuleFn, format_rule: F) -> String;
}

impl<T: VecString> VecStringNested for [T] {
    fn vec_string_nested(&self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&x.vec_string(inner_rule), i, l));
        }
        s
    }
}

impl<T: VecString, F: Fn(&str, usize, usize) -> String> VecStringFnNested<F> for [T] {
    fn vec_string_fn_nested(&self, inner_rule: FormatRuleFn, f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&x.vec_string(inner_rule), i, l));
        }
        s
    }
}

impl<T: VecString, F: FnMut(&str, usize, usize) -> String> VecStringFnMutNested<F> for [T] {
    fn vec_string_fn_mut_nested(&self, inner_rule: FormatRuleFn, mut f: F) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&f(&x.vec_string(inner_rule), i, l));
        }
        s
    }
}

// --- Iterator nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringNested {
    fn iter_string_nested(self, inner_rule: FormatRuleFn, format_rule: FormatRuleFn) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnNested<F: Fn(&str, usize, usize) -> String> {
    fn iter_string_fn_nested(self, inner_rule: FormatRuleFn, format_rule: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutNested<F: FnMut(&str, usize, usize) -> String> {
    fn iter_string_fn_mut_nested(self, inner_rule: FormatRuleFn, format_rule: F) -> String;
}

impl<I: StableIter> IteratorStringNested for I
where
    I::Item: VecString,
{
    fn iter_string_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

impl<I: StableIter, F: Fn(&str, usize, usize) -> String> IteratorStringFnNested<F> for I
where
    I::Item: VecString,
{
    fn iter_string_fn_nested(self, inner_rule: FormatRuleFn, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

impl<I: StableIter, F: FnMut(&str, usize, usize) -> String> IteratorStringFnMutNested<F> for I
where
    I::Item: VecString,
{
    fn iter_string_fn_mut_nested(self, inner_rule: FormatRuleFn, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

// --- WithState nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateNested<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn vec_string_with_state_nested(&self, inner_rule: FormatRuleFn, st: S, f: F) -> String;
}

impl<T: VecString, S, F: FnMut(&mut S, &str, usize, usize) -> String> VecStringWithStateNested<S, F>
    for [T]
{
    fn vec_string_with_state_nested(
        &self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut f: F,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateNested<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn iter_string_with_state_nested(self, inner_rule: FormatRuleFn, st: S, f: F) -> String;
}

impl<I: StableIter, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    IteratorStringWithStateNested<S, F> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut f: F,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

// --- WithStateFn nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateFnNested<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn vec_string_with_state_fn_nested(&self, inner_rule: FormatRuleFn, st: &S, f: F) -> String;
}

impl<T: VecString, S, F: Fn(&S, &str, usize, usize) -> String> VecStringWithStateFnNested<S, F>
    for [T]
{
    fn vec_string_with_state_fn_nested(&self, inner_rule: FormatRuleFn, st: &S, f: F) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFnNested<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn iter_string_with_state_fn_nested(self, inner_rule: FormatRuleFn, st: &S, f: F) -> String;
}

impl<I: StableIter, S, F: Fn(&S, &str, usize, usize) -> String>
    IteratorStringWithStateFnNested<S, F> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_fn_nested(self, inner_rule: FormatRuleFn, st: &S, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

// --- WithStateFnPtr nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateFnPtrNested<S> {
    fn vec_string_with_state_fn_ptr_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

impl<T: VecString, S> VecStringWithStateFnPtrNested<S> for [T] {
    fn vec_string_with_state_fn_ptr_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFnPtrNested<S> {
    fn iter_string_with_state_fn_ptr_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

impl<I: StableIter, S> IteratorStringWithStateFnPtrNested<S> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_fn_ptr_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

// --- RuleOwned nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringRuleOwnedNested<R: FormatRuleNoStateOwned> {
    fn vec_string_rule_owned_nested(&self, inner_rule: FormatRuleFn, rule: R) -> String;
}

impl<T: VecString, R: FormatRuleNoStateOwned + Clone> VecStringRuleOwnedNested<R> for [T] {
    fn vec_string_rule_owned_nested(&self, inner_rule: FormatRuleFn, rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.clone().format(&x.vec_string(inner_rule), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleOwnedNested<R: FormatRuleNoStateOwned> {
    fn iter_string_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

impl<I: StableIter, R: FormatRuleNoStateOwned + Clone> IteratorStringRuleOwnedNested<R> for I
where
    I::Item: VecString,
{
    fn iter_string_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.clone().format(&s, i, l));
        }
        r
    }
}

// --- MutRuleOwned nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringMutRuleOwnedNested<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_owned_nested(&self, inner_rule: FormatRuleFn, rule: R) -> String;
}

impl<T: VecString, R: FormatRuleMutNoState> VecStringMutRuleOwnedNested<R> for [T] {
    fn vec_string_mut_rule_owned_nested(&self, inner_rule: FormatRuleFn, mut rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&x.vec_string(inner_rule), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleOwnedNested<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

impl<I: StableIter, R: FormatRuleMutNoState> IteratorStringMutRuleOwnedNested<R> for I
where
    I::Item: VecString,
{
    fn iter_string_mut_rule_owned_nested(self, inner_rule: FormatRuleFn, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

// --- WithStateRuleOwned nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateRuleOwnedNested<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_owned_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String;
}

impl<T: VecString, S, R: FormatRule<S>> VecStringWithStateRuleOwnedNested<S, R> for [T] {
    fn vec_string_with_state_rule_owned_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleOwnedNested<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String;
}

impl<I: StableIter, S, R: FormatRule<S>> IteratorStringWithStateRuleOwnedNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

// --- WithStateMutRuleOwned nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateMutRuleOwnedNested<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_owned_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String;
}

impl<T: VecString, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleOwnedNested<S, R> for [T] {
    fn vec_string_with_state_mut_rule_owned_nested(
        &self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut rule: R,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleOwnedNested<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String;
}

impl<I: StableIter, S, R: FormatRuleMut<S>> IteratorStringWithStateMutRuleOwnedNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// --- RuleRef nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringRuleRefNested<'a, R: FormatRuleNoState> {
    fn vec_string_rule_ref_nested(&self, inner_rule: FormatRuleFn, rule: &'a R) -> String;
}

impl<'a, T: VecString, R: FormatRuleNoState> VecStringRuleRefNested<'a, R> for [T] {
    fn vec_string_rule_ref_nested(&self, inner_rule: FormatRuleFn, rule: &'a R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&x.vec_string(inner_rule), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleRefNested<'a, R: FormatRuleNoState> {
    fn iter_string_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String;
}

impl<'a, I: StableIter, R: FormatRuleNoState> IteratorStringRuleRefNested<'a, R> for I
where
    I::Item: VecString,
{
    fn iter_string_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

// --- MutRuleRef nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringMutRuleRefNested<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_ref_nested(&self, inner_rule: FormatRuleFn, rule: &mut R) -> String;
}

impl<T: VecString, R: FormatRuleMutNoState> VecStringMutRuleRefNested<R> for [T] {
    fn vec_string_mut_rule_ref_nested(&self, inner_rule: FormatRuleFn, rule: &mut R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&x.vec_string(inner_rule), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleRefNested<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &mut R) -> String;
}

impl<I: StableIter, R: FormatRuleMutNoState> IteratorStringMutRuleRefNested<R> for I
where
    I::Item: VecString,
{
    fn iter_string_mut_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

// --- WithStateRuleRef nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateRuleRefNested<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_ref_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String;
}

impl<T: VecString, S, R: FormatRule<S>> VecStringWithStateRuleRefNested<S, R> for [T] {
    fn vec_string_with_state_rule_ref_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleRefNested<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String;
}

impl<I: StableIter, S, R: FormatRule<S>> IteratorStringWithStateRuleRefNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

// --- WithStateMutRuleRef nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateMutRuleRefNested<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_ref_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: &mut R,
    ) -> String;
}

impl<T: VecString, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleRefNested<S, R> for [T] {
    fn vec_string_with_state_mut_rule_ref_nested(
        &self,
        inner_rule: FormatRuleFn,
        mut st: S,
        rule: &mut R,
    ) -> String {
        let mut r = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleRefNested<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: &mut R,
    ) -> String;
}

impl<I: StableIter, S, R: FormatRuleMut<S>> IteratorStringWithStateMutRuleRefNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        rule: &mut R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// --- Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringExactNested {
    fn iter_string_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        format_rule: FormatRuleFn,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator> IteratorStringExactNested for I
where
    I::Item: VecString,
{
    fn iter_string_exact_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&x.vec_string(inner_rule), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnExactNested<F: Fn(&str, usize, usize) -> String> {
    fn iter_string_fn_exact_nested(self, inner_rule: FormatRuleFn, format_rule: F) -> String;
}

impl<I: StableIter + ExactSizeIterator, F: Fn(&str, usize, usize) -> String>
    IteratorStringFnExactNested<F> for I
where
    I::Item: VecString,
{
    fn iter_string_fn_exact_nested(self, inner_rule: FormatRuleFn, f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&x.vec_string(inner_rule), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringFnMutExactNested<F: FnMut(&str, usize, usize) -> String> {
    fn iter_string_fn_mut_exact_nested(self, inner_rule: FormatRuleFn, format_rule: F) -> String;
}

impl<I: StableIter + ExactSizeIterator, F: FnMut(&str, usize, usize) -> String>
    IteratorStringFnMutExactNested<F> for I
where
    I::Item: VecString,
{
    fn iter_string_fn_mut_exact_nested(self, inner_rule: FormatRuleFn, mut f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&x.vec_string(inner_rule), i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateExactNested<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn iter_string_with_state_exact_nested(self, inner_rule: FormatRuleFn, st: S, f: F) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    IteratorStringWithStateExactNested<S, F> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut f: F,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFnExactNested<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn iter_string_with_state_fn_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: F,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, F: Fn(&S, &str, usize, usize) -> String>
    IteratorStringWithStateFnExactNested<S, F> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_fn_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: F,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFnPtrExactNested<S> {
    fn iter_string_with_state_fn_ptr_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, S> IteratorStringWithStateFnPtrExactNested<S> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_fn_ptr_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&f(st, &s, i, l));
        }
        r
    }
}

// --- RuleOwned Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleOwnedExactNested<R: FormatRuleNoStateOwned> {
    fn iter_string_rule_owned_exact_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

impl<I: StableIter + ExactSizeIterator, R: FormatRuleNoStateOwned + Clone>
    IteratorStringRuleOwnedExactNested<R> for I
where
    I::Item: VecString,
{
    fn iter_string_rule_owned_exact_nested(self, inner_rule: FormatRuleFn, rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.clone().format(&x.vec_string(inner_rule), i, l));
        }
        r
    }
}

// --- MutRuleOwned Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleOwnedExactNested<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_owned_exact_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

impl<I: StableIter + ExactSizeIterator, R: FormatRuleMutNoState>
    IteratorStringMutRuleOwnedExactNested<R> for I
where
    I::Item: VecString,
{
    fn iter_string_mut_rule_owned_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        mut rule: R,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&x.vec_string(inner_rule), i, l));
        }
        r
    }
}

// --- WithStateRuleOwned Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleOwnedExactNested<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_owned_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRule<S>>
    IteratorStringWithStateRuleOwnedExactNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_rule_owned_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

// --- WithStateMutRuleOwned Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleOwnedExactNested<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_owned_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRuleMut<S>>
    IteratorStringWithStateMutRuleOwnedExactNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_mut_rule_owned_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut rule: R,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// --- RuleRef Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleRefExactNested<'a, R: FormatRuleNoState> {
    fn iter_string_rule_ref_exact_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String;
}

impl<'a, I: StableIter + ExactSizeIterator, R: FormatRuleNoState>
    IteratorStringRuleRefExactNested<'a, R> for I
where
    I::Item: VecString,
{
    fn iter_string_rule_ref_exact_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&x.vec_string(inner_rule), i, l));
        }
        r
    }
}

// --- MutRuleRef Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringMutRuleRefExactNested<R: FormatRuleMutNoState> {
    fn iter_string_mut_rule_ref_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        rule: &mut R,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, R: FormatRuleMutNoState>
    IteratorStringMutRuleRefExactNested<R> for I
where
    I::Item: VecString,
{
    fn iter_string_mut_rule_ref_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        rule: &mut R,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&x.vec_string(inner_rule), i, l));
        }
        r
    }
}

// --- WithStateRuleRef Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateRuleRefExactNested<S, R: FormatRule<S>> {
    fn iter_string_with_state_rule_ref_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRule<S>>
    IteratorStringWithStateRuleRefExactNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_rule_ref_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(st, &s, i, l));
        }
        r
    }
}

// --- WithStateMutRuleRef Exact nested traits ---
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateMutRuleRefExactNested<S, R: FormatRuleMut<S>> {
    fn iter_string_with_state_mut_rule_ref_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: &mut R,
    ) -> String;
}

impl<I: StableIter + ExactSizeIterator, S, R: FormatRuleMut<S>>
    IteratorStringWithStateMutRuleRefExactNested<S, R> for I
where
    I::Item: VecString,
{
    fn iter_string_with_state_mut_rule_ref_exact_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        rule: &mut R,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = x.vec_string(inner_rule);
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// DISPLAY TRAITS для всех VecString* и IteratorString* (синхронных, collecting)
// ============================================================================
// --- VecString display traits ---
pub trait DisplayVecString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: FormatRuleFn) -> core::fmt::Result;
}
impl<T: core::fmt::Display> DisplayVecString for [T] {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: FormatRuleFn) -> core::fmt::Result {
        write!(f, "{}", self.vec_string(rule))
    }
}

pub trait DisplayVecStringFn<F: Fn(&str, usize, usize) -> String> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result;
}
impl<T: core::fmt::Display, F: Fn(&str, usize, usize) -> String> DisplayVecStringFn<F> for [T] {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_fn(rule))
    }
}

pub trait DisplayVecStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result;
}
impl<T: core::fmt::Display, F: FnMut(&str, usize, usize) -> String> DisplayVecStringFnMut<F>
    for [T]
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_fn_mut(rule))
    }
}

pub trait DisplayVecStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: F) -> core::fmt::Result;
}
impl<T: core::fmt::Display, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    DisplayVecStringWithState<S, F> for [T]
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state(st, rule))
    }
}

pub trait DisplayVecStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: F) -> core::fmt::Result;
}
impl<T: core::fmt::Display, S, F: Fn(&S, &str, usize, usize) -> String>
    DisplayVecStringWithStateFn<S, F> for [T]
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_fn(st, rule))
    }
}

pub trait DisplayVecStringWithStateFnPtr<S> {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        st: &S,
        rule: fn(&S, &str, usize, usize) -> String,
    ) -> core::fmt::Result;
}
impl<T: core::fmt::Display, S> DisplayVecStringWithStateFnPtr<S> for [T] {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        st: &S,
        rule: fn(&S, &str, usize, usize) -> String,
    ) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_fn_ptr(st, rule))
    }
}

// --- Display traits for RuleOwned / MutRuleOwned etc. ---
pub trait DisplayVecStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result;
}
impl<T: core::fmt::Display, R: FormatRuleNoStateOwned + Clone> DisplayVecStringRuleOwned<R>
    for [T]
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_rule_owned(rule))
    }
}

pub trait DisplayVecStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result;
}
impl<T: core::fmt::Display, R: FormatRuleMutNoState> DisplayVecStringMutRuleOwned<R> for [T] {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_mut_rule_owned(rule))
    }
}

pub trait DisplayVecStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: R) -> core::fmt::Result;
}
impl<T: core::fmt::Display, S, R: FormatRule<S>> DisplayVecStringWithStateRuleOwned<S, R>
    for [T]
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_rule_owned(st, rule))
    }
}

pub trait DisplayVecStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: R) -> core::fmt::Result;
}
impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> DisplayVecStringWithStateMutRuleOwned<S, R>
    for [T]
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_mut_rule_owned(st, rule))
    }
}

pub trait DisplayVecStringRuleRef<'a, R: FormatRuleNoState> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: &'a R) -> core::fmt::Result;
}
impl<'a, T: core::fmt::Display, R: FormatRuleNoState> DisplayVecStringRuleRef<'a, R> for [T] {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: &'a R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_rule_ref(rule))
    }
}

pub trait DisplayVecStringMutRuleRef<R: FormatRuleMutNoState> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: &mut R) -> core::fmt::Result;
}
impl<T: core::fmt::Display, R: FormatRuleMutNoState> DisplayVecStringMutRuleRef<R> for [T] {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: &mut R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_mut_rule_ref(rule))
    }
}

pub trait DisplayVecStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: &R) -> core::fmt::Result;
}
impl<T: core::fmt::Display, S, R: FormatRule<S>> DisplayVecStringWithStateRuleRef<S, R> for [T] {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: &R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_rule_ref(st, rule))
    }
}

pub trait DisplayVecStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: &mut R) -> core::fmt::Result;
}
impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> DisplayVecStringWithStateMutRuleRef<S, R>
    for [T]
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: &mut R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_mut_rule_ref(st, rule))
    }
}

// --- Display traits for iterators (collecting) ---
pub trait DisplayIteratorString {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: FormatRuleFn) -> core::fmt::Result;
}
impl<I: StableIter> DisplayIteratorString for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: FormatRuleFn) -> core::fmt::Result {
        write!(f, "{}", self.iter_string(rule))
    }
}

pub trait DisplayIteratorStringFn<F: Fn(&str, usize, usize) -> String> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result;
}
impl<I: StableIter, F: Fn(&str, usize, usize) -> String> DisplayIteratorStringFn<F> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_fn(rule))
    }
}

pub trait DisplayIteratorStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result;
}
impl<I: StableIter, F: FnMut(&str, usize, usize) -> String> DisplayIteratorStringFnMut<F> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_fn_mut(rule))
    }
}

pub trait DisplayIteratorStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: S, rule: F) -> core::fmt::Result;
}
impl<I: StableIter, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    DisplayIteratorStringWithState<S, F> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: S, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_with_state(st, rule))
    }
}

pub trait DisplayIteratorStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: F) -> core::fmt::Result;
}
impl<I: StableIter, S, F: Fn(&S, &str, usize, usize) -> String>
    DisplayIteratorStringWithStateFn<S, F> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_with_state_fn(st, rule))
    }
}

pub trait DisplayIteratorStringWithStateFnPtr<S> {
    fn fmt(
        self,
        f: &mut core::fmt::Formatter<'_>,
        st: &S,
        rule: fn(&S, &str, usize, usize) -> String,
    ) -> core::fmt::Result;
}
impl<I: StableIter, S> DisplayIteratorStringWithStateFnPtr<S> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(
        self,
        f: &mut core::fmt::Formatter<'_>,
        st: &S,
        rule: fn(&S, &str, usize, usize) -> String,
    ) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_with_state_fn_ptr(st, rule))
    }
}

pub trait DisplayIteratorStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result;
}
impl<I: StableIter, R: FormatRuleNoStateOwned + Clone> DisplayIteratorStringRuleOwned<R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_rule_owned(rule))
    }
}

pub trait DisplayIteratorStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result;
}
impl<I: StableIter, R: FormatRuleMutNoState> DisplayIteratorStringMutRuleOwned<R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_mut_rule_owned(rule))
    }
}

pub trait DisplayIteratorStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: R) -> core::fmt::Result;
}
impl<I: StableIter, S, R: FormatRule<S>> DisplayIteratorStringWithStateRuleOwned<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_with_state_rule_owned(st, rule))
    }
}

pub trait DisplayIteratorStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: S, rule: R) -> core::fmt::Result;
}
impl<I: StableIter, S, R: FormatRuleMut<S>> DisplayIteratorStringWithStateMutRuleOwned<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: S, rule: R) -> core::fmt::Result {
        write!(
            f,
            "{}",
            self.iter_string_with_state_mut_rule_owned(st, rule)
        )
    }
}

pub trait DisplayIteratorStringRuleRef<'a, R: FormatRuleNoState> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: &'a R) -> core::fmt::Result;
}
impl<'a, I: StableIter, R: FormatRuleNoState> DisplayIteratorStringRuleRef<'a, R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: &'a R) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_rule_ref(rule))
    }
}

pub trait DisplayIteratorStringMutRuleRef<R: FormatRuleMutNoState> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: &mut R) -> core::fmt::Result;
}
impl<I: StableIter, R: FormatRuleMutNoState> DisplayIteratorStringMutRuleRef<R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, rule: &mut R) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_mut_rule_ref(rule))
    }
}

pub trait DisplayIteratorStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: &R) -> core::fmt::Result;
}
impl<I: StableIter, S, R: FormatRule<S>> DisplayIteratorStringWithStateRuleRef<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: &R) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_with_state_rule_ref(st, rule))
    }
}

pub trait DisplayIteratorStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: S, rule: &mut R) -> core::fmt::Result;
}
impl<I: StableIter, S, R: FormatRuleMut<S>> DisplayIteratorStringWithStateMutRuleRef<S, R> for I
where
    I::Item: core::fmt::Display,
{
    fn fmt(self, f: &mut core::fmt::Formatter<'_>, st: S, rule: &mut R) -> core::fmt::Result {
        write!(f, "{}", self.iter_string_with_state_mut_rule_ref(st, rule))
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn vec_string_with_state(&self, st: S, f: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn iter_string_with_state(self, st: S, f: F) -> String;
}

impl<T: core::fmt::Display, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    VecStringWithState<S, F> for [T]
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn vec_string_with_state_fn(&self, st: &S, f: F) -> String;
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn iter_string_with_state_fn(self, st: &S, f: F) -> String;
}

impl<T: core::fmt::Display, S, F: Fn(&S, &str, usize, usize) -> String> VecStringWithStateFn<S, F>
    for [T]
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

impl<T: core::fmt::Display, S> VecStringWithStateFnPtr<S> for [T] {
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
// SYNC RuleOwned / MutRuleOwned / RuleRef / MutRuleRef (collecting)
// ============================================================================
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn vec_string_rule_owned(&self, rule: R) -> String;
}

impl<T: core::fmt::Display, R: FormatRuleNoStateOwned + Clone> VecStringRuleOwned<R> for [T] {
    fn vec_string_rule_owned(&self, rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.clone().format(&format!("{}", x), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_owned(&self, rule: R) -> String;
}

impl<T: core::fmt::Display, R: FormatRuleMutNoState> VecStringMutRuleOwned<R> for [T] {
    fn vec_string_mut_rule_owned(&self, mut rule: R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&format!("{}", x), i, l));
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_owned(&self, st: &S, rule: R) -> String;
}

impl<T: core::fmt::Display, S, R: FormatRule<S>> VecStringWithStateRuleOwned<S, R> for [T] {
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_owned(&self, st: S, rule: R) -> String;
}

impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleOwned<S, R> for [T] {
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringRuleRef<'a, R: FormatRuleNoState> {
    fn vec_string_rule_ref(&self, rule: &'a R) -> String;
}

impl<'a, T: core::fmt::Display, R: FormatRuleNoState> VecStringRuleRef<'a, R> for [T] {
    fn vec_string_rule_ref(&self, rule: &'a R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&format!("{}", x), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringMutRuleRef<R: FormatRuleMutNoState> {
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String;
}

impl<T: core::fmt::Display, R: FormatRuleMutNoState> VecStringMutRuleRef<R> for [T] {
    fn vec_string_mut_rule_ref(&self, rule: &mut R) -> String {
        let mut s = String::new();
        let l = self.len();
        for (i, x) in self.iter().enumerate() {
            s.push_str(&rule.format(&format!("{}", x), i, l));
        }
        s
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleRef<'a, R: FormatRuleNoState> {
    fn iter_string_rule_ref(self, rule: &'a R) -> String;
}

impl<'a, I: StableIter, R: FormatRuleNoState> IteratorStringRuleRef<'a, R> for I
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn vec_string_with_state_rule_ref(&self, st: &S, rule: &R) -> String;
}

impl<T: core::fmt::Display, S, R: FormatRule<S>> VecStringWithStateRuleRef<S, R> for [T] {
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait VecStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn vec_string_with_state_mut_rule_ref(&self, st: S, rule: &mut R) -> String;
}

impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleRef<S, R> for [T] {
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

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
    I::Item: core::fmt::Display,
{
    fn iter_string_exact(self, f: FormatRuleFn) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&format!("{}", x), i, l));
        }
        r
    }
}

impl<I: StableIter + ExactSizeIterator, F: Fn(&str, usize, usize) -> String>
    IteratorStringFnExact<F> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_fn_exact(self, f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&format!("{}", x), i, l));
        }
        r
    }
}

impl<I: StableIter + ExactSizeIterator, F: FnMut(&str, usize, usize) -> String>
    IteratorStringFnMutExact<F> for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_fn_mut_exact(self, mut f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&f(&format!("{}", x), i, l));
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
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_exact(self, mut st: S, mut f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_fn_exact(self, st: &S, f: F) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_fn_ptr_exact(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
{
    fn iter_string_rule_owned_exact(self, rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.clone().format(&format!("{}", x), i, l));
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
    I::Item: core::fmt::Display,
{
    fn iter_string_mut_rule_owned_exact(self, mut rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&format!("{}", x), i, l));
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
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_rule_owned_exact(self, st: &S, rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_mut_rule_owned_exact(self, mut st: S, mut rule: R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = format!("{}", x);
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait IteratorStringRuleRefExact<'a, R: FormatRuleNoState> {
    fn iter_string_rule_ref_exact(self, rule: &'a R) -> String;
}

impl<'a, I: StableIter + ExactSizeIterator, R: FormatRuleNoState> IteratorStringRuleRefExact<'a, R>
    for I
where
    I::Item: core::fmt::Display,
{
    fn iter_string_rule_ref_exact(self, rule: &'a R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&format!("{}", x), i, l));
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
    I::Item: core::fmt::Display,
{
    fn iter_string_mut_rule_ref_exact(self, rule: &mut R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            r.push_str(&rule.format(&format!("{}", x), i, l));
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
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_rule_ref_exact(self, st: &S, rule: &R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
{
    fn iter_string_with_state_mut_rule_ref_exact(self, mut st: S, rule: &mut R) -> String {
        let l = self.len();
        let mut r = String::new();
        for (i, x) in self.enumerate() {
            let s = format!("{}", x);
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
            .collect::<Vec<_>>()
            .concat()
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
            .collect::<Vec<_>>()
            .concat()
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
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
            .collect::<Vec<_>>()
            .concat()
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
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
            .collect::<Vec<_>>()
            .concat()
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
            .collect::<Vec<_>>()
            .concat()
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
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringRuleRef<'a, R: FormatRuleNoState> {
    fn par_iter_string_rule_ref(self, rule: &'a R) -> String;
}

#[cfg(feature = "rayon")]
impl<'a, I: rayon::iter::ParallelIterator, T: core::fmt::Display, R: FormatRuleNoState + Sync>
    ParIteratorStringRuleRef<'a, R> for I
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
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
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
// RAYON NESTED - recursive formatting for parallel iterators
// ============================================================================
#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringNested {
    fn par_iter_string_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString> ParIteratorStringNested for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(&s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnNested<F: Fn(&str, usize, usize) -> String> {
    fn par_iter_string_fn_nested(self, inner_rule: FormatRuleFn, f: F) -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: VecString,
        F: Fn(&str, usize, usize) -> String + Sync,
    > ParIteratorStringFnNested<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn_nested(self, inner_rule: FormatRuleFn, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(&s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringFnMutNested<F: FnMut(&str, usize, usize) -> String> {
    fn par_iter_string_fn_mut_nested(self, inner_rule: FormatRuleFn, f: F) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, F: FnMut(&str, usize, usize) -> String>
    ParIteratorStringFnMutNested<F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn_mut_nested(self, inner_rule: FormatRuleFn, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
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
pub trait ParIteratorStringFnPtrNested {
    fn par_iter_string_fn_ptr_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString> ParIteratorStringFnPtrNested for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_fn_ptr_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(&s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateNested<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn par_iter_string_with_state_nested(self, inner_rule: FormatRuleFn, st: S, f: F) -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: VecString,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> String,
    > ParIteratorStringWithStateNested<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut f: F,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
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
pub trait ParIteratorStringWithStateFnNested<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn par_iter_string_with_state_fn_nested(self, inner_rule: FormatRuleFn, st: &S, f: F)
        -> String;
}

#[cfg(feature = "rayon")]
impl<
        I: rayon::iter::ParallelIterator,
        T: VecString,
        S: Sync,
        F: Fn(&S, &str, usize, usize) -> String + Sync,
    > ParIteratorStringWithStateFnNested<S, F> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_fn_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: F,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(st, &s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateFnPtrNested<S> {
    fn par_iter_string_with_state_fn_ptr_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, S: Sync>
    ParIteratorStringWithStateFnPtrNested<S> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_fn_ptr_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| f(st, &s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringRuleOwnedNested<R: FormatRuleNoStateOwned> {
    fn par_iter_string_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, R: FormatRuleNoStateOwned + Clone + Sync>
    ParIteratorStringRuleOwnedNested<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.clone().format(&s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringMutRuleOwnedNested<R: FormatRuleMutNoState> {
    fn par_iter_string_mut_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, R: FormatRuleMutNoState>
    ParIteratorStringMutRuleOwnedNested<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        mut rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
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
pub trait ParIteratorStringWithStateRuleOwnedNested<S, R: FormatRule<S>> {
    fn par_iter_string_with_state_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, S: Sync, R: FormatRule<S> + Sync>
    ParIteratorStringWithStateRuleOwnedNested<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.format(st, &s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateMutRuleOwnedNested<S, R: FormatRuleMut<S>> {
    fn par_iter_string_with_state_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, S, R: FormatRuleMut<S>>
    ParIteratorStringWithStateMutRuleOwnedNested<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
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
pub trait ParIteratorStringRuleRefNested<'a, R: FormatRuleNoState> {
    fn par_iter_string_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String;
}

#[cfg(feature = "rayon")]
impl<'a, I: rayon::iter::ParallelIterator, T: VecString, R: FormatRuleNoState + Sync>
    ParIteratorStringRuleRefNested<'a, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.format(&s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringMutRuleRefNested<R: FormatRuleMutNoState> {
    fn par_iter_string_mut_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &mut R) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, R: FormatRuleMutNoState>
    ParIteratorStringMutRuleRefNested<R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_mut_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
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
pub trait ParIteratorStringWithStateRuleRefNested<S, R: FormatRule<S>> {
    fn par_iter_string_with_state_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, S: Sync, R: FormatRule<S> + Sync>
    ParIteratorStringWithStateRuleRefNested<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        items
            .into_par_iter()
            .enumerate()
            .map(|(i, s)| rule.format(st, &s, i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "rayon")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait ParIteratorStringWithStateMutRuleRefNested<S, R: FormatRuleMut<S>> {
    fn par_iter_string_with_state_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: &mut R,
    ) -> String;
}

#[cfg(feature = "rayon")]
impl<I: rayon::iter::ParallelIterator, T: VecString, S, R: FormatRuleMut<S>>
    ParIteratorStringWithStateMutRuleRefNested<S, R> for I
where
    I: rayon::iter::ParallelIterator<Item = T>,
{
    fn par_iter_string_with_state_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        rule: &mut R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// ORX-PARALLEL SYNC
// ============================================================================
#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorString {
    fn orx_par_iter_string(self, f: FormatRuleFn) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display> OrxParIteratorString for P {
    fn orx_par_iter_string(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFn<F: Fn(&str, usize, usize) -> String> {
    fn orx_par_iter_string_fn(self, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> String + Sync + Clone,
    > OrxParIteratorStringFn<F> for P
{
    fn orx_par_iter_string_fn(self, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMut<F: FnMut(&str, usize, usize) -> String> {
    fn orx_par_iter_string_fn_mut(self, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> String,
    > OrxParIteratorStringFnMut<F> for P
{
    fn orx_par_iter_string_fn_mut(self, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnPtr {
    fn orx_par_iter_string_fn_ptr(self, f: FormatRuleFn) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display> OrxParIteratorStringFnPtr for P {
    fn orx_par_iter_string_fn_ptr(self, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithState<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn orx_par_iter_string_with_state(self, st: S, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> String,
    > OrxParIteratorStringWithState<S, F> for P
{
    fn orx_par_iter_string_with_state(self, mut st: S, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateFn<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn orx_par_iter_string_with_state_fn(self, st: &S, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        S: Sync,
        F: Fn(&S, &str, usize, usize) -> String + Sync + Clone,
    > OrxParIteratorStringWithStateFn<S, F> for P
{
    fn orx_par_iter_string_with_state_fn(self, st: &S, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateFnPtr<S> {
    fn orx_par_iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display, S: Sync>
    OrxParIteratorStringWithStateFnPtr<S> for P
{
    fn orx_par_iter_string_with_state_fn_ptr(
        self,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringRuleOwned<R: FormatRuleNoStateOwned> {
    fn orx_par_iter_string_rule_owned(self, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        R: FormatRuleNoStateOwned + Clone + Sync,
    > OrxParIteratorStringRuleOwned<R> for P
{
    fn orx_par_iter_string_rule_owned(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.clone().format(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringMutRuleOwned<R: FormatRuleMutNoState> {
    fn orx_par_iter_string_mut_rule_owned(self, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display, R: FormatRuleMutNoState>
    OrxParIteratorStringMutRuleOwned<R> for P
{
    fn orx_par_iter_string_mut_rule_owned(self, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateRuleOwned<S, R: FormatRule<S>> {
    fn orx_par_iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        S: Sync,
        R: FormatRule<S> + Clone + Sync,
    > OrxParIteratorStringWithStateRuleOwned<S, R> for P
{
    fn orx_par_iter_string_with_state_rule_owned(self, st: &S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.format(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateMutRuleOwned<S, R: FormatRuleMut<S>> {
    fn orx_par_iter_string_with_state_mut_rule_owned(self, st: S, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display, S, R: FormatRuleMut<S>>
    OrxParIteratorStringWithStateMutRuleOwned<S, R> for P
{
    fn orx_par_iter_string_with_state_mut_rule_owned(self, mut st: S, mut rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringRuleRef<'a, R: FormatRuleNoState> {
    fn orx_par_iter_string_rule_ref(self, rule: &'a R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        'a,
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        R: FormatRuleNoState + Sync,
    > OrxParIteratorStringRuleRef<'a, R> for P
{
    fn orx_par_iter_string_rule_ref(self, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.format(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringMutRuleRef<R: FormatRuleMutNoState> {
    fn orx_par_iter_string_mut_rule_ref(self, rule: &mut R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display, R: FormatRuleMutNoState>
    OrxParIteratorStringMutRuleRef<R> for P
{
    fn orx_par_iter_string_mut_rule_ref(self, rule: &mut R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateRuleRef<S, R: FormatRule<S>> {
    fn orx_par_iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        S: Sync,
        R: FormatRule<S> + Sync,
    > OrxParIteratorStringWithStateRuleRef<S, R> for P
{
    fn orx_par_iter_string_with_state_rule_ref(self, st: &S, rule: &R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.format(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateMutRuleRef<S, R: FormatRuleMut<S>> {
    fn orx_par_iter_string_with_state_mut_rule_ref(self, st: S, rule: &mut R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display, S, R: FormatRuleMut<S>>
    OrxParIteratorStringWithStateMutRuleRef<S, R> for P
{
    fn orx_par_iter_string_with_state_mut_rule_ref(self, mut st: S, rule: &mut R) -> String {
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
// ORX-PARALLEL NESTED - recursive formatting for parallel iterators
// ============================================================================
#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringNested {
    fn orx_par_iter_string_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString> OrxParIteratorStringNested for P {
    fn orx_par_iter_string_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnNested<F: Fn(&str, usize, usize) -> String> {
    fn orx_par_iter_string_fn_nested(self, inner_rule: FormatRuleFn, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        F: Fn(&str, usize, usize) -> String + Sync + Clone,
    > OrxParIteratorStringFnNested<F> for P
{
    fn orx_par_iter_string_fn_nested(self, inner_rule: FormatRuleFn, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutNested<F: FnMut(&str, usize, usize) -> String> {
    fn orx_par_iter_string_fn_mut_nested(self, inner_rule: FormatRuleFn, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, F: FnMut(&str, usize, usize) -> String>
    OrxParIteratorStringFnMutNested<F> for P
{
    fn orx_par_iter_string_fn_mut_nested(self, inner_rule: FormatRuleFn, mut f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnPtrNested {
    fn orx_par_iter_string_fn_ptr_nested(self, inner_rule: FormatRuleFn, f: FormatRuleFn)
        -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString> OrxParIteratorStringFnPtrNested for P {
    fn orx_par_iter_string_fn_ptr_nested(
        self,
        inner_rule: FormatRuleFn,
        f: FormatRuleFn,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateNested<S, F: FnMut(&mut S, &str, usize, usize) -> String> {
    fn orx_par_iter_string_with_state_nested(self, inner_rule: FormatRuleFn, st: S, f: F)
        -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> String,
    > OrxParIteratorStringWithStateNested<S, F> for P
{
    fn orx_par_iter_string_with_state_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut f: F,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateFnNested<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn orx_par_iter_string_with_state_fn_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: F,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        S: Sync,
        F: Fn(&S, &str, usize, usize) -> String + Sync + Clone,
    > OrxParIteratorStringWithStateFnNested<S, F> for P
{
    fn orx_par_iter_string_with_state_fn_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: F,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateFnPtrNested<S> {
    fn orx_par_iter_string_with_state_fn_ptr_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, S: Sync>
    OrxParIteratorStringWithStateFnPtrNested<S> for P
{
    fn orx_par_iter_string_with_state_fn_ptr_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| f(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringRuleOwnedNested<R: FormatRuleNoStateOwned> {
    fn orx_par_iter_string_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        R: FormatRuleNoStateOwned + Clone + Sync,
    > OrxParIteratorStringRuleOwnedNested<R> for P
{
    fn orx_par_iter_string_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.clone().format(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringMutRuleOwnedNested<R: FormatRuleMutNoState> {
    fn orx_par_iter_string_mut_rule_owned_nested(self, inner_rule: FormatRuleFn, rule: R)
        -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, R: FormatRuleMutNoState>
    OrxParIteratorStringMutRuleOwnedNested<R> for P
{
    fn orx_par_iter_string_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        mut rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateRuleOwnedNested<S, R: FormatRule<S>> {
    fn orx_par_iter_string_with_state_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        S: Sync,
        R: FormatRule<S> + Clone + Sync,
    > OrxParIteratorStringWithStateRuleOwnedNested<S, R> for P
{
    fn orx_par_iter_string_with_state_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.format(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateMutRuleOwnedNested<S, R: FormatRuleMut<S>> {
    fn orx_par_iter_string_with_state_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, S, R: FormatRuleMut<S>>
    OrxParIteratorStringWithStateMutRuleOwnedNested<S, R> for P
{
    fn orx_par_iter_string_with_state_mut_rule_owned_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        mut rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringRuleRefNested<'a, R: FormatRuleNoState> {
    fn orx_par_iter_string_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<'a, P: orx_parallel::ParIter<Item = T>, T: VecString, R: FormatRuleNoState + Sync>
    OrxParIteratorStringRuleRefNested<'a, R> for P
{
    fn orx_par_iter_string_rule_ref_nested(self, inner_rule: FormatRuleFn, rule: &'a R) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.format(&items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringMutRuleRefNested<R: FormatRuleMutNoState> {
    fn orx_par_iter_string_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        rule: &mut R,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, R: FormatRuleMutNoState>
    OrxParIteratorStringMutRuleRefNested<R> for P
{
    fn orx_par_iter_string_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        rule: &mut R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateRuleRefNested<S, R: FormatRule<S>> {
    fn orx_par_iter_string_with_state_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, S: Sync, R: FormatRule<S> + Sync>
    OrxParIteratorStringWithStateRuleRefNested<S, R> for P
{
    fn orx_par_iter_string_with_state_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        (0..l)
            .into_par()
            .map(|i| rule.format(st, &items[i], i, l))
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateMutRuleRefNested<S, R: FormatRuleMut<S>> {
    fn orx_par_iter_string_with_state_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: &mut R,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, S, R: FormatRuleMut<S>>
    OrxParIteratorStringWithStateMutRuleRefNested<S, R> for P
{
    fn orx_par_iter_string_with_state_mut_rule_ref_nested(
        self,
        inner_rule: FormatRuleFn,
        mut st: S,
        rule: &mut R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.format(&mut st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// ORX-PARALLEL + DYN ASYNC / IMPL ASYNC
// ============================================================================
#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnAsync<'a, F, Fut> {
    fn orx_par_iter_string_async_fn(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: core::future::Future<Output = String> + 'a,
        F: Fn(&str, usize, usize) -> Fut + Sync;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<'a, P, T, F, Fut> OrxParIteratorStringFnAsync<'a, F, Fut> for P
where
    P: orx_parallel::ParIter<Item = T>,
    T: core::fmt::Display,
{
    fn orx_par_iter_string_async_fn(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: core::future::Future<Output = String> + 'a,
        F: Fn(&str, usize, usize) -> Fut + Sync,
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

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnAsyncSend<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send,
        T: core::fmt::Display + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnAsyncSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutAsync<
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn orx_par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > OrxParIteratorStringFnMutAsync<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutAsyncSend<
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> Box<dyn core::future::Future<Output = String> + 'a + Send>
    where
        Self: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send,
        T: core::fmt::Display + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnMutAsyncSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnPtrAsync {
    fn orx_par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display> OrxParIteratorStringFnPtrAsync
    for P
{
    fn orx_par_iter_string_async_fn_ptr<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnImplAsync<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String>,
>
{
    fn orx_par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String>,
    > OrxParIteratorStringFnImplAsync<F, Fut> for P
{
    fn orx_par_iter_string_async_fn<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnImplAsyncSend<
    F: Fn(&str, usize, usize) -> Fut + Sync,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn<'a>(
        self,
        f: &'a F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send,
        T: core::fmt::Display + Send,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnImplAsyncSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutImplAsync<
    F: FnMut(&str, usize, usize) -> Fut,
    Fut: core::future::Future<Output = String>,
>
{
    fn orx_par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > OrxParIteratorStringFnMutImplAsync<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutImplAsyncSend<
    F: FnMut(&str, usize, usize) -> Fut + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn_mut<'a>(
        self,
        f: &'a mut F,
    ) -> impl core::future::Future<Output = String> + 'a + Send
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send,
        T: core::fmt::Display + Send,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnMutImplAsyncSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut<'a>(
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

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnPtrImplAsync {
    fn orx_par_iter_string_async_fn_ptr<'a>(
        self,
        f: FormatRuleFn,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display> OrxParIteratorStringFnPtrImplAsync
    for P
{
    fn orx_par_iter_string_async_fn_ptr<'a>(
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
// ORX-PARALLEL SYNC CLONE - F: Clone instead of F: Sync (no shared reference)
// ============================================================================
#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnClone<F: Fn(&str, usize, usize) -> String> {
    fn orx_par_iter_string_fn_clone(self, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> String + Clone,
    > OrxParIteratorStringFnClone<F> for P
{
    fn orx_par_iter_string_fn_clone(self, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f.clone()(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateFnClone<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn orx_par_iter_string_with_state_fn_clone(self, st: S, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        S: Clone,
        F: Fn(&S, &str, usize, usize) -> String + Clone,
    > OrxParIteratorStringWithStateFnClone<S, F> for P
{
    fn orx_par_iter_string_with_state_fn_clone(self, st: S, f: F) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f.clone()(&st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringRuleRefClone<R: FormatRuleNoState> {
    fn orx_par_iter_string_rule_ref_clone(self, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: core::fmt::Display, R: FormatRuleNoState + Clone>
    OrxParIteratorStringRuleRefClone<R> for P
{
    fn orx_par_iter_string_rule_ref_clone(self, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.clone().format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateRuleRefClone<S, R: FormatRule<S>> {
    fn orx_par_iter_string_with_state_rule_ref_clone(self, st: S, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        S: Clone,
        R: FormatRule<S> + Clone,
    > OrxParIteratorStringWithStateRuleRefClone<S, R> for P
{
    fn orx_par_iter_string_with_state_rule_ref_clone(self, st: S, rule: R) -> String {
        let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.clone().format(&st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// ORX-PARALLEL NESTED CLONE - F: Clone instead of F: Sync
// ============================================================================
#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnCloneNested<F: Fn(&str, usize, usize) -> String> {
    fn orx_par_iter_string_fn_clone_nested(self, inner_rule: FormatRuleFn, f: F) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        F: Fn(&str, usize, usize) -> String + Clone,
    > OrxParIteratorStringFnCloneNested<F> for P
{
    fn orx_par_iter_string_fn_clone_nested(self, inner_rule: FormatRuleFn, f: F) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f.clone()(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateFnCloneNested<S, F: Fn(&S, &str, usize, usize) -> String> {
    fn orx_par_iter_string_with_state_fn_clone_nested(self, inner_rule: FormatRuleFn, st: S, f: F)
        -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        S: Clone,
        F: Fn(&S, &str, usize, usize) -> String + Clone,
    > OrxParIteratorStringWithStateFnCloneNested<S, F> for P
{
    fn orx_par_iter_string_with_state_fn_clone_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        f: F,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&f.clone()(&st, &s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringRuleRefCloneNested<R: FormatRuleNoState> {
    fn orx_par_iter_string_rule_ref_clone_nested(self, inner_rule: FormatRuleFn, rule: R) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<P: orx_parallel::ParIter<Item = T>, T: VecString, R: FormatRuleNoState + Clone>
    OrxParIteratorStringRuleRefCloneNested<R> for P
{
    fn orx_par_iter_string_rule_ref_clone_nested(
        self,
        inner_rule: FormatRuleFn,
        rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.clone().format(&s, i, l));
        }
        r
    }
}

#[cfg(feature = "orx_parallel")]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringWithStateRuleRefCloneNested<S, R: FormatRule<S>> {
    fn orx_par_iter_string_with_state_rule_ref_clone_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String;
}

#[cfg(feature = "orx_parallel")]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: VecString,
        S: Clone,
        R: FormatRule<S> + Clone,
    > OrxParIteratorStringWithStateRuleRefCloneNested<S, R> for P
{
    fn orx_par_iter_string_with_state_rule_ref_clone_nested(
        self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String {
        let items: Vec<String> = self.map(|x| x.vec_string(inner_rule)).collect();
        let l = items.len();
        let mut r = String::new();
        for (i, s) in items.into_iter().enumerate() {
            r.push_str(&rule.clone().format(&st, &s, i, l));
        }
        r
    }
}

// ============================================================================
// ORX-PARALLEL + DYN ASYNC CLONE - F: Clone, no Sync required
// ============================================================================
#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnAsyncClone<
    F: Fn(&str, usize, usize) -> Fut + Clone,
    Fut: core::future::Future<Output = String>,
>
{
    fn orx_par_iter_string_async_fn_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String>>;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + 'static,
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> Fut + Clone + 'static,
        Fut: core::future::Future<Output = String>,
    > OrxParIteratorStringFnAsyncClone<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String>> {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnAsyncCloneSend<
    F: Fn(&str, usize, usize) -> Fut + Clone + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String> + Send>;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send + 'static,
        T: core::fmt::Display + Send,
        F: Fn(&str, usize, usize) -> Fut + Clone + Send + 'static,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnAsyncCloneSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String> + Send> {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutAsyncClone<
    F: FnMut(&str, usize, usize) -> Fut + Clone,
    Fut: core::future::Future<Output = String>,
>
{
    fn orx_par_iter_string_async_fn_mut_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String>>;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + 'static,
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> Fut + Clone + 'static,
        Fut: core::future::Future<Output = String>,
    > OrxParIteratorStringFnMutAsyncClone<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String>> {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        })
    }
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutAsyncCloneSend<
    F: FnMut(&str, usize, usize) -> Fut + Clone + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn_mut_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String> + Send>;
}

#[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send + 'static,
        T: core::fmt::Display + Send,
        F: FnMut(&str, usize, usize) -> Fut + Clone + Send + 'static,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnMutAsyncCloneSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut_clone(
        self,
        f: F,
    ) -> Box<dyn core::future::Future<Output = String> + Send> {
        Box::new(async move {
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
            let l = items.len();
            let mut r = String::new();
            r.reserve(l.saturating_mul(16));
            for (i, s) in items.into_iter().enumerate() {
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        })
    }
}

// ============================================================================
// ORX-PARALLEL + IMPL ASYNC CLONE - F: Clone, no Sync required
// ============================================================================
#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnImplAsyncClone<
    F: Fn(&str, usize, usize) -> Fut + Clone,
    Fut: core::future::Future<Output = String>,
>
{
    fn orx_par_iter_string_async_fn_clone<'a>(
        self,
        f: F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> Fut + Clone,
        Fut: core::future::Future<Output = String>,
    > OrxParIteratorStringFnImplAsyncClone<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_clone<'a>(
        self,
        f: F,
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
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnImplAsyncCloneSend<
    F: Fn(&str, usize, usize) -> Fut + Clone + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn_clone<'a>(
        self,
        f: F,
    ) -> impl core::future::Future<Output = String> + Send + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send,
        T: core::fmt::Display + Send,
        F: Fn(&str, usize, usize) -> Fut + Clone + Send,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnImplAsyncCloneSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_clone<'a>(
        self,
        f: F,
    ) -> impl core::future::Future<Output = String> + Send + 'a
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
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutImplAsyncClone<
    F: FnMut(&str, usize, usize) -> Fut + Clone,
    Fut: core::future::Future<Output = String>,
>
{
    fn orx_par_iter_string_async_fn_mut_clone<'a>(
        self,
        f: F,
    ) -> impl core::future::Future<Output = String> + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T>,
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> Fut + Clone,
        Fut: core::future::Future<Output = String>,
    > OrxParIteratorStringFnMutImplAsyncClone<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut_clone<'a>(
        self,
        f: F,
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
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        }
    }
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
#[cfg_attr(feature = "ambassador_delegatable", ambassador::delegatable_trait)]
pub trait OrxParIteratorStringFnMutImplAsyncCloneSend<
    F: FnMut(&str, usize, usize) -> Fut + Clone + Send,
    Fut: core::future::Future<Output = String> + Send,
>
{
    fn orx_par_iter_string_async_fn_mut_clone<'a>(
        self,
        f: F,
    ) -> impl core::future::Future<Output = String> + Send + 'a
    where
        Self: 'a,
        F: 'a,
        Fut: 'a;
}

#[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
impl<
        P: orx_parallel::ParIter<Item = T> + Send,
        T: core::fmt::Display + Send,
        F: FnMut(&str, usize, usize) -> Fut + Clone + Send,
        Fut: core::future::Future<Output = String> + Send,
    > OrxParIteratorStringFnMutImplAsyncCloneSend<F, Fut> for P
{
    fn orx_par_iter_string_async_fn_mut_clone<'a>(
        self,
        f: F,
    ) -> impl core::future::Future<Output = String> + Send + 'a
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
                r.push_str(&f.clone()(&s, i, l).await);
            }
            r
        }
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
// DYN ASYNC: Vec (collecting)
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
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > VecStringFnAsync<'a, F, Fut> for [T]
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
        T: core::fmt::Display + Sync,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > VecStringFnAsyncSend<'a, F, Fut> for [T]
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
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > VecStringFnMutAsync<'a, F, Fut> for [T]
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
        T: core::fmt::Display + Sync,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + 'a + Send,
    > VecStringFnMutAsyncSend<'a, F, Fut> for [T]
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
        T: core::fmt::Display,
        S: 'a,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String> + 'a,
    > VecStringWithStateAsync<'a, S, F, Fut> for [T]
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
    I::Item: core::fmt::Display,
{
    fn iter_string_async_fn(self, f: &'a F) -> Box<dyn core::future::Future<Output = String> + 'a>
    where
        Self: 'a,
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
    I::Item: core::fmt::Display,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display + Send,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display + Send,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
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
        T: core::fmt::Display,
        F: Fn(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > VecStringFnImplAsync<F, Fut> for [T]
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
        T: core::fmt::Display + Sync,
        F: Fn(&str, usize, usize) -> Fut + Sync,
        Fut: core::future::Future<Output = String> + Send,
    > VecStringFnImplAsyncSend<F, Fut> for [T]
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
        T: core::fmt::Display,
        F: FnMut(&str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > VecStringFnMutImplAsync<F, Fut> for [T]
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
        T: core::fmt::Display + Sync,
        F: FnMut(&str, usize, usize) -> Fut + Send,
        Fut: core::future::Future<Output = String> + Send,
    > VecStringFnMutImplAsyncSend<F, Fut> for [T]
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
        T: core::fmt::Display,
        S,
        F: FnMut(&mut S, &str, usize, usize) -> Fut,
        Fut: core::future::Future<Output = String>,
    > VecStringWithStateImplAsync<S, F, Fut> for [T]
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
    I::Item: core::fmt::Display,
{
    fn iter_string_async_fn<'a>(self, f: &'a F) -> impl core::future::Future<Output = String> + 'a
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
            let items: Vec<String> = self.map(|x| format!("{}", x)).collect();
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
    I::Item: core::fmt::Display,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display + Send,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display + Send,
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
                let s = format!("{}", x);
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
    I::Item: core::fmt::Display,
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
                let s = format!("{}", x);
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
    T: core::fmt::Display,
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
// ADDITIONAL IMPL FOR Vec<T>
// ============================================================================

impl<T: core::fmt::Display> DisplayVecString for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: FormatRuleFn) -> core::fmt::Result {
        write!(f, "{}", self.vec_string(rule))
    }
}

impl<T: core::fmt::Display, F: Fn(&str, usize, usize) -> String> DisplayVecStringFn<F> for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_fn(rule))
    }
}

impl<T: core::fmt::Display, F: FnMut(&str, usize, usize) -> String> DisplayVecStringFnMut<F>
    for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_fn_mut(rule))
    }
}

impl<T: core::fmt::Display, S, F: FnMut(&mut S, &str, usize, usize) -> String>
    DisplayVecStringWithState<S, F> for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state(st, rule))
    }
}

impl<T: core::fmt::Display, S, F: Fn(&S, &str, usize, usize) -> String>
    DisplayVecStringWithStateFn<S, F> for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: F) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_fn(st, rule))
    }
}

impl<T: core::fmt::Display, S> DisplayVecStringWithStateFnPtr<S> for Vec<T> {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        st: &S,
        rule: fn(&S, &str, usize, usize) -> String,
    ) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_fn_ptr(st, rule))
    }
}

impl<T: core::fmt::Display, R: FormatRuleNoStateOwned + Clone> DisplayVecStringRuleOwned<R>
    for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_rule_owned(rule))
    }
}

impl<T: core::fmt::Display, R: FormatRuleMutNoState> DisplayVecStringMutRuleOwned<R> for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_mut_rule_owned(rule))
    }
}

impl<T: core::fmt::Display, S, R: FormatRule<S>> DisplayVecStringWithStateRuleOwned<S, R>
    for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_rule_owned(st, rule))
    }
}

impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> DisplayVecStringWithStateMutRuleOwned<S, R>
    for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_mut_rule_owned(st, rule))
    }
}

impl<'a, T: core::fmt::Display, R: FormatRuleNoState> DisplayVecStringRuleRef<'a, R> for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: &'a R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_rule_ref(rule))
    }
}

impl<T: core::fmt::Display, R: FormatRuleMutNoState> DisplayVecStringMutRuleRef<R> for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, rule: &mut R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_mut_rule_ref(rule))
    }
}

impl<T: core::fmt::Display, S, R: FormatRule<S>> DisplayVecStringWithStateRuleRef<S, R> for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: &S, rule: &R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_rule_ref(st, rule))
    }
}

impl<T: core::fmt::Display, S, R: FormatRuleMut<S>> DisplayVecStringWithStateMutRuleRef<S, R>
    for Vec<T>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, st: S, rule: &mut R) -> core::fmt::Result {
        write!(f, "{}", self.vec_string_with_state_mut_rule_ref(st, rule))
    }
}

// ============================================================================
// NESTED Vec<T> IMPLS - delegate to [T] nested impls via as_slice()
// ============================================================================

impl<T: VecString> VecStringNested for Vec<T> {
    fn vec_string_nested(&self, inner_rule: FormatRuleFn, f: FormatRuleFn) -> String {
        self.as_slice().vec_string_nested(inner_rule, f)
    }
}

impl<T: VecString, F: Fn(&str, usize, usize) -> String> VecStringFnNested<F> for Vec<T> {
    fn vec_string_fn_nested(&self, inner_rule: FormatRuleFn, f: F) -> String {
        self.as_slice().vec_string_fn_nested(inner_rule, f)
    }
}

impl<T: VecString, F: FnMut(&str, usize, usize) -> String> VecStringFnMutNested<F> for Vec<T> {
    fn vec_string_fn_mut_nested(&self, inner_rule: FormatRuleFn, f: F) -> String {
        self.as_slice().vec_string_fn_mut_nested(inner_rule, f)
    }
}

impl<T: VecString, S, F: FnMut(&mut S, &str, usize, usize) -> String> VecStringWithStateNested<S, F>
    for Vec<T>
{
    fn vec_string_with_state_nested(&self, inner_rule: FormatRuleFn, st: S, f: F) -> String {
        self.as_slice()
            .vec_string_with_state_nested(inner_rule, st, f)
    }
}

impl<T: VecString, S, F: Fn(&S, &str, usize, usize) -> String> VecStringWithStateFnNested<S, F>
    for Vec<T>
{
    fn vec_string_with_state_fn_nested(&self, inner_rule: FormatRuleFn, st: &S, f: F) -> String {
        self.as_slice()
            .vec_string_with_state_fn_nested(inner_rule, st, f)
    }
}

impl<T: VecString, S> VecStringWithStateFnPtrNested<S> for Vec<T> {
    fn vec_string_with_state_fn_ptr_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        f: fn(&S, &str, usize, usize) -> String,
    ) -> String {
        self.as_slice()
            .vec_string_with_state_fn_ptr_nested(inner_rule, st, f)
    }
}

impl<T: VecString, R: FormatRuleNoStateOwned + Clone> VecStringRuleOwnedNested<R> for Vec<T> {
    fn vec_string_rule_owned_nested(&self, inner_rule: FormatRuleFn, rule: R) -> String {
        self.as_slice()
            .vec_string_rule_owned_nested(inner_rule, rule)
    }
}

impl<T: VecString, R: FormatRuleMutNoState> VecStringMutRuleOwnedNested<R> for Vec<T> {
    fn vec_string_mut_rule_owned_nested(&self, inner_rule: FormatRuleFn, rule: R) -> String {
        self.as_slice()
            .vec_string_mut_rule_owned_nested(inner_rule, rule)
    }
}

impl<T: VecString, S, R: FormatRule<S>> VecStringWithStateRuleOwnedNested<S, R> for Vec<T> {
    fn vec_string_with_state_rule_owned_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: R,
    ) -> String {
        self.as_slice()
            .vec_string_with_state_rule_owned_nested(inner_rule, st, rule)
    }
}

impl<T: VecString, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleOwnedNested<S, R> for Vec<T> {
    fn vec_string_with_state_mut_rule_owned_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: R,
    ) -> String {
        self.as_slice()
            .vec_string_with_state_mut_rule_owned_nested(inner_rule, st, rule)
    }
}

impl<'a, T: VecString, R: FormatRuleNoState> VecStringRuleRefNested<'a, R> for Vec<T> {
    fn vec_string_rule_ref_nested(&self, inner_rule: FormatRuleFn, rule: &'a R) -> String {
        self.as_slice().vec_string_rule_ref_nested(inner_rule, rule)
    }
}

impl<T: VecString, R: FormatRuleMutNoState> VecStringMutRuleRefNested<R> for Vec<T> {
    fn vec_string_mut_rule_ref_nested(&self, inner_rule: FormatRuleFn, rule: &mut R) -> String {
        self.as_slice()
            .vec_string_mut_rule_ref_nested(inner_rule, rule)
    }
}

impl<T: VecString, S, R: FormatRule<S>> VecStringWithStateRuleRefNested<S, R> for Vec<T> {
    fn vec_string_with_state_rule_ref_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: &S,
        rule: &R,
    ) -> String {
        self.as_slice()
            .vec_string_with_state_rule_ref_nested(inner_rule, st, rule)
    }
}

impl<T: VecString, S, R: FormatRuleMut<S>> VecStringWithStateMutRuleRefNested<S, R> for Vec<T> {
    fn vec_string_with_state_mut_rule_ref_nested(
        &self,
        inner_rule: FormatRuleFn,
        st: S,
        rule: &mut R,
    ) -> String {
        self.as_slice()
            .vec_string_with_state_mut_rule_ref_nested(inner_rule, st, rule)
    }
}

// ============================================================================
// ASYNC Vec impls for Vec<T>
// ============================================================================

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

#[cfg(test)]
mod tests {
    fn rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
        DEFAULT_FORMAT_RULE(v, i, l)
    }
    const RULE_PTR: fn(&i32, &str, usize, usize) -> String = rule;
    use super::*;
    use alloc::format;
    use alloc::string::String;
    use alloc::vec;
    #[cfg(any(feature = "dyn_async", feature = "impl_async"))]
    use core::future::Future;
    use pollster::*;

    // ========================================================================
    // Async Block Helpers
    // ========================================================================

    #[cfg(feature = "dyn_async")]
    fn block_on_dyn<'a, T>(fut: Box<dyn Future<Output = T> + 'a>) -> T {
        let mut pin_future = Box::into_pin(fut);
        let fut = pin_future.as_mut();
        fut.block_on()
    }

    // ========================================================================
    // 1. SYNC VEC BASE TRAITS
    // ========================================================================
    #[test]
    fn test_sync_vec_base_traits() {
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", v.vec_string(DEFAULT_FORMAT_RULE));
        assert_eq!("[1, 2, 3]", v.vec_string_fn(DEFAULT_FORMAT_RULE));
        assert_eq!("[1, 2, 3]", v.vec_string_fn_mut(DEFAULT_FORMAT_RULE));

        let state = 0i32;
        assert_eq!(
            "[1, 2, 3]",
            v.vec_string_with_state(state, |s, v, i, l| {
                *s += 1;
                DEFAULT_FORMAT_RULE(v, i, l)
            })
        );
        assert_eq!(
            "[1, 2, 3]",
            v.vec_string_with_state_fn(&state, |_s, v, i, l| DEFAULT_FORMAT_RULE(v, i, l))
        );

        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        assert_eq!(
            "[1, 2, 3]",
            v.vec_string_with_state_fn_ptr(&state, ptr_rule)
        );
    }

    // ========================================================================
    // 2. SYNC VEC RULE TRAITS
    // ========================================================================
    #[test]
    fn test_sync_vec_rule_owned_traits() {
        let v = [1, 2, 3];
        let r1 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.vec_string_rule_owned(r1));

        let mut r2 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.vec_string_mut_rule_owned(&mut r2));

        let r3 = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.vec_string_with_state_rule_owned(&0i32, r3));

        let mut r4 = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        assert_eq!(
            "[1, 2, 3]",
            v.vec_string_with_state_mut_rule_owned(0i32, &mut r4)
        );
    }

    #[test]
    fn test_sync_vec_rule_ref_traits() {
        let v = [1, 2, 3];
        let r1 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.vec_string_rule_ref(&r1));

        let mut r2 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.vec_string_mut_rule_ref(&mut r2));

        let r3 = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.vec_string_with_state_rule_ref(&0i32, &r3));

        let mut r4 = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        assert_eq!(
            "[1, 2, 3]",
            v.vec_string_with_state_mut_rule_ref(0i32, &mut r4)
        );
    }

    // ========================================================================
    // 3. SYNC ITER BASE TRAITS
    // ========================================================================
    #[test]
    fn test_sync_iter_base_traits() {
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", v.iter().iter_string(DEFAULT_FORMAT_RULE));
        assert_eq!("[1, 2, 3]", v.iter().iter_string_fn(DEFAULT_FORMAT_RULE));
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_fn_mut(DEFAULT_FORMAT_RULE)
        );

        let state = 0i32;
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state(state, |s, v, i, l| {
                *s += 1;
                DEFAULT_FORMAT_RULE(v, i, l)
            })
        );
        assert_eq!(
            "[1, 2, 3]",
            v.iter()
                .iter_string_with_state_fn(&state, |_s, v, i, l| DEFAULT_FORMAT_RULE(v, i, l))
        );

        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state_fn_ptr(&state, ptr_rule)
        );
    }

    // ========================================================================
    // 4. SYNC ITER RULE TRAITS
    // ========================================================================
    #[test]
    fn test_sync_iter_rule_owned_traits() {
        let v = [1, 2, 3];
        let r1 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.iter().iter_string_rule_owned(r1));

        let mut r2 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.iter().iter_string_mut_rule_owned(&mut r2));

        let r3 = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state_rule_owned(&0i32, r3)
        );

        let mut r4 = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        assert_eq!(
            "[1, 2, 3]",
            v.iter()
                .iter_string_with_state_mut_rule_owned(0i32, &mut r4)
        );
    }

    #[test]
    fn test_sync_iter_rule_ref_traits() {
        let v = [1, 2, 3];
        let r1 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.iter().iter_string_rule_ref(&r1));

        let mut r2 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.iter().iter_string_mut_rule_ref(&mut r2));

        let r3 = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state_rule_ref(&0i32, &r3)
        );

        let mut r4 = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state_mut_rule_ref(0i32, &mut r4)
        );
    }

    // ========================================================================
    // 5. EXACT SIZE ITERATOR TRAITS
    // ========================================================================
    #[test]
    fn test_exact_size_base_traits() {
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", v.iter().iter_string_exact(DEFAULT_FORMAT_RULE));
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_fn_exact(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_fn_mut_exact(DEFAULT_FORMAT_RULE)
        );

        let state = 0i32;
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state_exact(state, |s, v, i, l| {
                *s += 1;
                DEFAULT_FORMAT_RULE(v, i, l)
            })
        );
        assert_eq!(
            "[1, 2, 3]",
            v.iter()
                .iter_string_with_state_fn_exact(&state, |_s, v, i, l| DEFAULT_FORMAT_RULE(
                    v, i, l
                ))
        );

        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        assert_eq!(
            "[1, 2, 3]",
            v.iter()
                .iter_string_with_state_fn_ptr_exact(&state, ptr_rule)
        );
    }

    #[test]
    fn test_exact_size_rule_traits() {
        let v = [1, 2, 3];
        let r1 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.iter().iter_string_rule_owned_exact(r1));
        assert_eq!("[1, 2, 3]", v.iter().iter_string_rule_ref_exact(&r1));

        let mut r2 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_mut_rule_owned_exact(&mut r2)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_mut_rule_ref_exact(&mut r2)
        );

        let r3 = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state_rule_owned_exact(&0i32, r3)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.iter().iter_string_with_state_rule_ref_exact(&0i32, &r3)
        );

        let mut r4 = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        assert_eq!(
            "[1, 2, 3]",
            v.iter()
                .iter_string_with_state_mut_rule_owned_exact(0i32, &mut r4)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.iter()
                .iter_string_with_state_mut_rule_ref_exact(0i32, &mut r4)
        );
    }

    // ========================================================================
    // 6. DISPLAY TRAITS (28 individual tests)
    // ========================================================================

    // --- Vec Display Traits (14) ---

    #[test]
    fn test_display_vec_string() {
        struct W<'a>(&'a Vec<i32>);
        impl<'a> core::fmt::Display for W<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecString::fmt(self.0, f, DEFAULT_FORMAT_RULE)
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v)));
    }

    #[test]
    fn test_display_vec_string_fn() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, F: Clone>(&'a Vec<i32>, F);
        impl<'a, F: Fn(&str, usize, usize) -> String + Clone> core::fmt::Display for W<'a, F> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringFn::fmt(self.0, f, self.1.clone())
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, rule)));
    }

    #[test]
    fn test_display_vec_string_fn_mut() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, F: Clone>(&'a Vec<i32>, F);
        impl<'a, F: FnMut(&str, usize, usize) -> String + Clone> core::fmt::Display for W<'a, F> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringFnMut::fmt(self.0, f, self.1.clone())
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, rule)));
    }

    #[test]
    fn test_display_vec_string_with_state() {
        let rule = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        struct W<'a, S: Clone, F: Clone>(&'a Vec<i32>, S, F);
        impl<'a, S: Clone, F: FnMut(&mut S, &str, usize, usize) -> String + Clone>
            core::fmt::Display for W<'a, S, F>
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringWithState::fmt(self.0, f, self.1.clone(), self.2.clone())
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, 0i32, rule)));
    }

    #[test]
    fn test_display_vec_string_with_state_fn() {
        let rule = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, S, F: Clone>(&'a Vec<i32>, S, F);
        impl<'a, S, F: Fn(&S, &str, usize, usize) -> String + Clone> core::fmt::Display for W<'a, S, F> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringWithStateFn::fmt(self.0, f, &self.1, self.2.clone())
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, 0i32, rule)));
    }

    #[test]
    fn test_display_vec_string_with_state_fn_ptr() {
        struct W<'a>(&'a Vec<i32>, i32);
        impl<'a> core::fmt::Display for W<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringWithStateFnPtr::fmt(self.0, f, &self.1, RULE_PTR)
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, 0i32)));
    }

    #[test]
    fn test_display_vec_string_rule_owned() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, R: Clone>(&'a Vec<i32>, R);
        impl<'a, R: FormatRuleNoStateOwned + Clone> core::fmt::Display for W<'a, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringRuleOwned::fmt(self.0, f, self.1.clone())
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, rule)));
    }

    #[test]
    fn test_display_vec_string_mut_rule_owned() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, R: Clone>(&'a Vec<i32>, R);
        impl<'a, R: FormatRuleMutNoState + Clone> core::fmt::Display for W<'a, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringMutRuleOwned::fmt(self.0, f, self.1.clone())
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, rule)));
    }

    #[test]
    fn test_display_vec_string_with_state_rule_owned() {
        let rule = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, S, R: Clone>(&'a Vec<i32>, S, R);
        impl<'a, S, R: FormatRule<S> + Clone> core::fmt::Display for W<'a, S, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringWithStateRuleOwned::fmt(self.0, f, &self.1, self.2.clone())
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, 0i32, rule)));
    }

    #[test]
    fn test_display_vec_string_with_state_mut_rule_owned() {
        let rule = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        struct W<'a, S: Clone, R: Clone>(&'a Vec<i32>, S, R);
        impl<'a, S: Clone, R: FormatRuleMut<S> + Clone> core::fmt::Display for W<'a, S, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringWithStateMutRuleOwned::fmt(
                    self.0,
                    f,
                    self.1.clone(),
                    self.2.clone(),
                )
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, 0i32, rule)));
    }

    #[test]
    fn test_display_vec_string_rule_ref() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, 'b, R>(&'a Vec<i32>, &'b R);
        impl<'a, 'b, R: FormatRuleNoState> core::fmt::Display for W<'a, 'b, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringRuleRef::fmt(self.0, f, self.1)
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, &rule)));
    }

    #[test]
    fn test_display_vec_string_mut_rule_ref() {
        let mut rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, R>(&'a Vec<i32>, *mut R);
        impl<'a, R: FormatRuleMutNoState> core::fmt::Display for W<'a, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringMutRuleRef::fmt(self.0, f, unsafe { &mut *self.1 })
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, &mut rule)));
    }

    #[test]
    fn test_display_vec_string_with_state_rule_ref() {
        let rule = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, 'b, S, R>(&'a Vec<i32>, S, &'b R);
        impl<'a, 'b, S, R: FormatRule<S>> core::fmt::Display for W<'a, 'b, S, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringWithStateRuleRef::fmt(self.0, f, &self.1, self.2)
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, 0i32, &rule)));
    }

    #[test]
    fn test_display_vec_string_with_state_mut_rule_ref() {
        let mut rule = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        struct W<'a, S: Clone, R>(&'a Vec<i32>, S, *mut R);
        impl<'a, S: Clone, R: FormatRuleMut<S>> core::fmt::Display for W<'a, S, R> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayVecStringWithStateMutRuleRef::fmt(self.0, f, self.1.clone(), unsafe {
                    &mut *self.2
                })
            }
        }
        let v = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v, 0i32, &mut rule)));
    }

    // --- Iterator Display Traits (14) ---

    #[test]
    fn test_display_iterator_string() {
        struct W<I: Clone>(I);
        impl<I: StableIter + Clone> core::fmt::Display for W<I>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorString::fmt(self.0.clone(), f, DEFAULT_FORMAT_RULE)
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter())));
    }

    #[test]
    fn test_display_iterator_string_fn() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<I: Clone, F: Clone>(I, F);
        impl<I: StableIter + Clone, F: Fn(&str, usize, usize) -> String + Clone> core::fmt::Display
            for W<I, F>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringFn::fmt(self.0.clone(), f, self.1.clone())
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), rule)));
    }

    #[test]
    fn test_display_iterator_string_fn_mut() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<I: Clone, F: Clone>(I, F);
        impl<I: StableIter + Clone, F: FnMut(&str, usize, usize) -> String + Clone>
            core::fmt::Display for W<I, F>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringFnMut::fmt(self.0.clone(), f, self.1.clone())
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), rule)));
    }

    #[test]
    fn test_display_iterator_string_with_state() {
        let rule = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        struct W<I: Clone, S: Clone, F: Clone>(I, S, F);
        impl<
                I: StableIter + Clone,
                S: Clone,
                F: FnMut(&mut S, &str, usize, usize) -> String + Clone,
            > core::fmt::Display for W<I, S, F>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringWithState::fmt(
                    self.0.clone(),
                    f,
                    self.1.clone(),
                    self.2.clone(),
                )
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), 0i32, rule)));
    }

    #[test]
    fn test_display_iterator_string_with_state_fn() {
        let rule = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<I: Clone, S, F: Clone>(I, S, F);
        impl<I: StableIter + Clone, S, F: Fn(&S, &str, usize, usize) -> String + Clone>
            core::fmt::Display for W<I, S, F>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringWithStateFn::fmt(self.0.clone(), f, &self.1, self.2.clone())
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), 0i32, rule)));
    }

    #[test]
    fn test_display_iterator_string_with_state_fn_ptr() {
        struct W<I: Clone>(I, i32);
        impl<I: StableIter + Clone> core::fmt::Display for W<I>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringWithStateFnPtr::fmt(self.0.clone(), f, &self.1, RULE_PTR)
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), 0i32)));
    }

    #[test]
    fn test_display_iterator_string_rule_owned() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<I: Clone, R: Clone>(I, R);
        impl<I: StableIter + Clone, R: FormatRuleNoStateOwned + Clone> core::fmt::Display for W<I, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringRuleOwned::fmt(self.0.clone(), f, self.1.clone())
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), rule)));
    }

    #[test]
    fn test_display_iterator_string_mut_rule_owned() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<I: Clone, R: Clone>(I, R);
        impl<I: StableIter + Clone, R: FormatRuleMutNoState + Clone> core::fmt::Display for W<I, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringMutRuleOwned::fmt(self.0.clone(), f, self.1.clone())
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), rule)));
    }

    #[test]
    fn test_display_iterator_string_with_state_rule_owned() {
        let rule = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<I: Clone, S, R: Clone>(I, S, R);
        impl<I: StableIter + Clone, S, R: FormatRule<S> + Clone> core::fmt::Display for W<I, S, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringWithStateRuleOwned::fmt(
                    self.0.clone(),
                    f,
                    &self.1,
                    self.2.clone(),
                )
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), 0i32, rule)));
    }

    #[test]
    fn test_display_iterator_string_with_state_mut_rule_owned() {
        let rule = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        struct W<I: Clone, S: Clone, R: Clone>(I, S, R);
        impl<I: StableIter + Clone, S: Clone, R: FormatRuleMut<S> + Clone> core::fmt::Display for W<I, S, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringWithStateMutRuleOwned::fmt(
                    self.0.clone(),
                    f,
                    self.1.clone(),
                    self.2.clone(),
                )
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), 0i32, rule)));
    }

    #[test]
    fn test_display_iterator_string_rule_ref() {
        let rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'a, I: Clone, R>(&'a I, &'a R);
        impl<'a, I: StableIter + Clone, R: FormatRuleNoState> core::fmt::Display for W<'a, I, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringRuleRef::fmt(self.0.clone(), f, self.1)
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v.iter(), &rule)));
    }

    #[test]
    fn test_display_iterator_string_mut_rule_ref() {
        let mut rule = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<I: Clone, R>(I, *mut R);
        impl<I: StableIter + Clone, R: FormatRuleMutNoState> core::fmt::Display for W<I, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringMutRuleRef::fmt(self.0.clone(), f, unsafe { &mut *self.1 })
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), &mut rule)));
    }

    #[test]
    fn test_display_iterator_string_with_state_rule_ref() {
        let rule = |_s: &i32, v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        struct W<'b, I: Clone, S, R>(&'b I, S, R);
        impl<'b, I: StableIter + Clone, S, R: FormatRule<S>> core::fmt::Display for W<'b, I, S, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringWithStateRuleRef::fmt(self.0.clone(), f, &self.1, &self.2)
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(&v.iter(), 0i32, rule)));
    }

    #[test]
    fn test_display_iterator_string_with_state_mut_rule_ref() {
        let mut rule = |s: &mut i32, v: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(v, i, l)
        };
        struct W<I: Clone, S: Clone, R>(I, S, *mut R);
        impl<I: StableIter + Clone, S: Clone, R: FormatRuleMut<S>> core::fmt::Display for W<I, S, R>
        where
            I::Item: core::fmt::Display,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                DisplayIteratorStringWithStateMutRuleRef::fmt(
                    self.0.clone(),
                    f,
                    self.1.clone(),
                    unsafe { &mut *self.2 },
                )
            }
        }
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", format!("{}", W(v.iter(), 0i32, &mut rule)));
    }

    // ========================================================================
    // 7. EXTENDED DISPLAY MARKER TRAIT
    // ========================================================================
    #[test]
    fn test_extended_display_marker() {
        let v = vec![1, 2, 3];
        fn takes_extended<T: ExtendedDisplay>(_x: T) {}
        takes_extended(v);
        takes_extended([1, 2, 3].iter());
    }

    // ========================================================================
    // 8. RAYON SYNC TRAITS
    // ========================================================================
    #[cfg(feature = "rayon")]
    #[test]
    fn test_rayon_sync_traits() {
        use rayon::prelude::*;
        let v = vec![1, 2, 3];
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter().par_iter_string(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter().par_iter_string_fn(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter().par_iter_string_fn_mut(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter().par_iter_string_fn_ptr(DEFAULT_FORMAT_RULE)
        );

        let state = 0i32;
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter()
                .par_iter_string_with_state(state, |s, v, i, l| {
                    *s += 1;
                    DEFAULT_FORMAT_RULE(v, i, l)
                })
        );
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter()
                .par_iter_string_with_state_fn(&state, |_s, v, i, l| DEFAULT_FORMAT_RULE(v, i, l))
        );

        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        assert_eq!(
            "[1, 2, 3]",
            v.par_iter()
                .par_iter_string_with_state_fn_ptr(&state, ptr_rule)
        );

        let r1 = |v: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(v, i, l);
        assert_eq!("[1, 2, 3]", v.par_iter().par_iter_string_rule_owned(r1));
        assert_eq!("[1, 2, 3]", v.par_iter().par_iter_string_rule_ref(&r1));
    }

    // ========================================================================
    // 9. DYN ASYNC TRAITS
    // ========================================================================
    #[cfg(feature = "dyn_async")]
    #[test]
    fn test_dyn_async_traits() {
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move { DEFAULT_FORMAT_RULE(&value, index, length) }
        };
        assert_eq!(
            "[1, 2, 3]",
            block_on_dyn(VecStringFnAsync::vec_string_async_fn(&v, &fmt))
        );
        assert_eq!(
            "[1, 2, 3]",
            block_on_dyn(IteratorStringFnAsync::iter_string_async_fn(v.iter(), &fmt))
        );
        assert_eq!(
            "[1, 2, 3]",
            block_on_dyn(IteratorStringFnAsyncExact::iter_string_async_fn_exact(
                v.iter(),
                &fmt
            ))
        );
    }

    // ========================================================================
    // 10. IMPL ASYNC TRAITS
    // ========================================================================
    #[cfg(feature = "impl_async")]
    #[test]
    fn test_impl_async_traits() {
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move { DEFAULT_FORMAT_RULE(&value, index, length) }
        };
        assert_eq!(
            "[1, 2, 3]",
            block_on(VecStringFnImplAsync::vec_string_async_fn(&v, &fmt))
        );
        assert_eq!(
            "[1, 2, 3]",
            block_on(IteratorStringFnImplAsync::iter_string_async_fn(
                v.iter(),
                &fmt
            ))
        );
        assert_eq!(
            "[1, 2, 3]",
            block_on(IteratorStringFnImplAsyncExact::iter_string_async_fn_exact(
                v.iter(),
                &fmt
            ))
        );
    }

    // ========================================================================
    // 11. RAYON + ASYNC TRAITS
    // ========================================================================
    #[cfg(all(feature = "rayon", feature = "dyn_async"))]
    #[test]
    fn test_rayon_dyn_async() {
        use rayon::prelude::*;
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move { DEFAULT_FORMAT_RULE(&value, index, length) }
        };
        assert_eq!(
            "[1, 2, 3]",
            block_on_dyn(ParIteratorStringFnAsync::par_iter_string_async_fn(
                v.par_iter(),
                &fmt
            ))
        );
    }

    #[cfg(all(feature = "rayon", feature = "impl_async"))]
    #[test]
    fn test_rayon_impl_async() {
        use rayon::prelude::*;
        let v = vec![1, 2, 3];
        let fmt = |value: &str, index: usize, length: usize| {
            let value = value.to_string();
            async move { DEFAULT_FORMAT_RULE(&value, index, length) }
        };
        assert_eq!(
            "[1, 2, 3]",
            block_on(ParIteratorStringFnImplAsync::par_iter_string_async_fn(
                v.par_iter(),
                &fmt
            ))
        );
    }

    // ========================================================================
    // 12. CORNER CASES
    // ========================================================================
    #[test]
    fn test_custom_format_rule_no_brackets() {
        let v = [1, 2, 3];
        let res = v.vec_string(|val, idx, total| {
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
        let fmt = |val: &str, _i: usize, _t: usize| format!("({})", val);
        let col_res = IteratorStringFn::iter_string_fn(data.iter(), fmt);
        let exact_res = IteratorStringFnExact::iter_string_fn_exact(data.iter(), fmt);
        assert_eq!(col_res, exact_res);
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use alloc::format;
    use alloc::string::String;
    use alloc::vec;

    // ========================================================================
    // Edge Cases
    // ========================================================================
    #[test]
    fn test_empty_vec() {
        let v: Vec<i32> = vec![];
        assert_eq!("", v.vec_string(DEFAULT_FORMAT_RULE));
        assert_eq!("", v.vec_string_fn(DEFAULT_FORMAT_RULE));
        assert_eq!("", v.vec_string_fn_mut(DEFAULT_FORMAT_RULE));
    }

    #[test]
    fn test_single_element_vec() {
        let v = [42];
        assert_eq!("[42]", v.vec_string(DEFAULT_FORMAT_RULE));
    }

    #[test]
    fn test_large_vec() {
        let v: Vec<i32> = (0..100).collect();
        let result = v.vec_string(DEFAULT_FORMAT_RULE);
        assert!(result.starts_with("[0, 1, 2"));
        assert!(result.ends_with(", 99]"));
    }

    #[test]
    fn test_two_elements_vec() {
        let v = [1, 2];
        assert_eq!("[1, 2]", v.vec_string(DEFAULT_FORMAT_RULE));
    }

    // ========================================================================
    // VecStringWithState Tests
    // ========================================================================
    #[test]
    fn test_vec_string_with_state_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let result = v.vec_string_with_state(&mut counter, |state, val, idx, len| {
            **state += 1;
            format!("{}({})", val, state)
        });
        assert_eq!(result, "1(1)2(2)3(3)");
    }

    #[test]
    fn test_vec_string_with_state_fn_basic() {
        let v = [1, 2, 3];
        let prefix = "item";
        let result = v.vec_string_with_state_fn(&prefix, |state, val, idx, len| {
            format!("{}: {}", state, val)
        });
        assert_eq!(result, "item: 1item: 2item: 3");
    }

    #[test]
    fn test_vec_string_with_state_fn_ptr_basic() {
        fn my_rule(state: &String, val: &str, idx: usize, len: usize) -> String {
            format!("{}={}", state, val)
        }
        let v = [1, 2, 3];
        let prefix = "val".to_string();
        let result = v.vec_string_with_state_fn_ptr(&prefix, my_rule);
        assert_eq!(result, "val=1val=2val=3");
    }

    // ========================================================================
    // Rule Owned Tests
    // ========================================================================
    #[test]
    fn test_vec_string_rule_owned_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("[{}]", val);
        assert_eq!("[1][2][3]", v.vec_string_rule_owned(rule));
    }

    #[test]
    fn test_vec_string_mut_rule_owned_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}#{}", val, counter)
        };
        assert_eq!("1#12#23#3", v.vec_string_mut_rule_owned(rule));
    }

    #[test]
    fn test_vec_string_with_state_rule_owned_basic() {
        let v = [1, 2, 3];
        let state = "prefix";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}:{}", s, val);
        assert_eq!(
            "prefix:1prefix:2prefix:3",
            v.vec_string_with_state_rule_owned(&state, rule)
        );
    }

    #[test]
    fn test_vec_string_with_state_mut_rule_owned_basic() {
        let v = [1, 2, 3];
        let mut state = 0;
        let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 1;
            format!("{}#{}", val, s)
        };
        assert_eq!(
            "1#12#23#3",
            v.vec_string_with_state_mut_rule_owned(&mut state, rule)
        );
    }

    // ========================================================================
    // Rule Ref Tests
    // ========================================================================
    #[test]
    fn test_vec_string_rule_ref_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("({})", val);
        assert_eq!("(1)(2)(3)", v.vec_string_rule_ref(&rule));
    }

    #[test]
    fn test_vec_string_mut_rule_ref_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let mut rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}[{}]", val, counter)
        };
        assert_eq!("1[1]2[2]3[3]", v.vec_string_mut_rule_ref(&mut rule));
    }

    #[test]
    fn test_vec_string_with_state_rule_ref_basic() {
        let v = [1, 2, 3];
        let state = "key";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}={}", s, val);
        assert_eq!(
            "key=1key=2key=3",
            v.vec_string_with_state_rule_ref(&state, &rule)
        );
    }

    #[test]
    fn test_vec_string_with_state_mut_rule_ref_basic() {
        let v = [1, 2, 3];
        let mut state = 100;
        let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 10;
            format!("{}+{}", val, s)
        };
        assert_eq!(
            "1+1102+1203+130",
            v.vec_string_with_state_mut_rule_ref(&mut state, &mut rule)
        );
    }

    // ========================================================================
    // Iterator Tests
    // ========================================================================
    #[test]
    fn test_iterator_string_basic() {
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", v.iter().iter_string(DEFAULT_FORMAT_RULE));
    }

    #[test]
    fn test_iterator_string_fn_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("{{{}}}", val);
        assert_eq!("{1}{2}{3}", v.iter().iter_string_fn(rule));
    }

    #[test]
    fn test_iterator_string_fn_mut_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}#{}", val, counter)
        };
        assert_eq!("1#12#23#3", v.iter().iter_string_fn_mut(rule));
    }

    #[test]
    fn test_iterator_string_with_state_basic() {
        let v = [1, 2, 3];
        let mut state = 0;
        let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 1;
            format!("{}({})", val, s)
        };
        assert_eq!(
            "1(1)2(2)3(3)",
            v.iter().iter_string_with_state(&mut state, rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_fn_basic() {
        let v = [1, 2, 3];
        let prefix = "item";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}: {}", s, val);
        assert_eq!(
            "item: 1item: 2item: 3",
            v.iter().iter_string_with_state_fn(&prefix, rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_fn_ptr_basic() {
        fn my_rule(state: &String, val: &str, idx: usize, len: usize) -> String {
            format!("{}={}", state, val)
        }
        let v = [1, 2, 3];
        let prefix = "val".to_string();
        assert_eq!(
            "val=1val=2val=3",
            v.iter().iter_string_with_state_fn_ptr(&prefix, my_rule)
        );
    }

    // ========================================================================
    // Iterator Rule Tests
    // ========================================================================
    #[test]
    fn test_iterator_string_rule_owned_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("[{}]", val);
        assert_eq!("[1][2][3]", v.iter().iter_string_rule_owned(rule));
    }

    #[test]
    fn test_iterator_string_mut_rule_owned_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}#{}", val, counter)
        };
        assert_eq!("1#12#23#3", v.iter().iter_string_mut_rule_owned(rule));
    }

    #[test]
    fn test_iterator_string_with_state_rule_owned_basic() {
        let v = [1, 2, 3];
        let state = "prefix";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}:{}", s, val);
        assert_eq!(
            "prefix:1prefix:2prefix:3",
            v.iter().iter_string_with_state_rule_owned(&state, rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule_owned_basic() {
        let v = [1, 2, 3];
        let mut state = 0;
        let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 1;
            format!("{}#{}", val, s)
        };
        assert_eq!(
            "1#12#23#3",
            v.iter()
                .iter_string_with_state_mut_rule_owned(&mut state, rule)
        );
    }

    #[test]
    fn test_iterator_string_rule_ref_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("({})", val);
        assert_eq!("(1)(2)(3)", v.iter().iter_string_rule_ref(&rule));
    }

    #[test]
    fn test_iterator_string_mut_rule_ref_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let mut rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}[{}]", val, counter)
        };
        assert_eq!("1[1]2[2]3[3]", v.iter().iter_string_mut_rule_ref(&mut rule));
    }

    #[test]
    fn test_iterator_string_with_state_rule_ref_basic() {
        let v = [1, 2, 3];
        let state = "key";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}={}", s, val);
        assert_eq!(
            "key=1key=2key=3",
            v.iter().iter_string_with_state_rule_ref(&state, &rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule_ref_basic() {
        let v = [1, 2, 3];
        let mut state = 100;
        let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 10;
            format!("{}+{}", val, s)
        };
        assert_eq!(
            "1+1102+1203+130",
            v.iter()
                .iter_string_with_state_mut_rule_ref(&mut state, &mut rule)
        );
    }

    // ========================================================================
    // ExactSizeIterator Tests
    // ========================================================================
    #[test]
    fn test_iterator_string_exact_basic() {
        let v = [1, 2, 3];
        assert_eq!("[1, 2, 3]", v.iter().iter_string_exact(DEFAULT_FORMAT_RULE));
    }

    #[test]
    fn test_iterator_string_fn_exact_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("{{{}}}", val);
        assert_eq!("{1}{2}{3}", v.iter().iter_string_fn_exact(rule));
    }

    #[test]
    fn test_iterator_string_fn_mut_exact_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}#{}", val, counter)
        };
        assert_eq!("1#12#23#3", v.iter().iter_string_fn_mut_exact(rule));
    }

    #[test]
    fn test_iterator_string_with_state_exact_basic() {
        let v = [1, 2, 3];
        let mut state = 0;
        let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 1;
            format!("{}({})", val, s)
        };
        assert_eq!(
            "1(1)2(2)3(3)",
            v.iter().iter_string_with_state_exact(&mut state, rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_fn_exact_basic() {
        let v = [1, 2, 3];
        let prefix = "item";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}: {}", s, val);
        assert_eq!(
            "item: 1item: 2item: 3",
            v.iter().iter_string_with_state_fn_exact(&prefix, rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_fn_ptr_exact_basic() {
        fn my_rule(state: &String, val: &str, idx: usize, len: usize) -> String {
            format!("{}={}", state, val)
        }
        let v = [1, 2, 3];
        let prefix = "val".to_string();
        assert_eq!(
            "val=1val=2val=3",
            v.iter()
                .iter_string_with_state_fn_ptr_exact(&prefix, my_rule)
        );
    }

    #[test]
    fn test_iterator_string_rule_owned_exact_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("[{}]", val);
        assert_eq!("[1][2][3]", v.iter().iter_string_rule_owned_exact(rule));
    }

    #[test]
    fn test_iterator_string_mut_rule_owned_exact_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}#{}", val, counter)
        };
        assert_eq!("1#12#23#3", v.iter().iter_string_mut_rule_owned_exact(rule));
    }

    #[test]
    fn test_iterator_string_with_state_rule_owned_exact_basic() {
        let v = [1, 2, 3];
        let state = "prefix";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}:{}", s, val);
        assert_eq!(
            "prefix:1prefix:2prefix:3",
            v.iter()
                .iter_string_with_state_rule_owned_exact(&state, rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule_owned_exact_basic() {
        let v = [1, 2, 3];
        let mut state = 0;
        let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 1;
            format!("{}#{}", val, s)
        };
        assert_eq!(
            "1#12#23#3",
            v.iter()
                .iter_string_with_state_mut_rule_owned_exact(&mut state, rule)
        );
    }

    #[test]
    fn test_iterator_string_rule_ref_exact_basic() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("({})", val);
        assert_eq!("(1)(2)(3)", v.iter().iter_string_rule_ref_exact(&rule));
    }

    #[test]
    fn test_iterator_string_mut_rule_ref_exact_basic() {
        let v = [1, 2, 3];
        let mut counter = 0;
        let mut rule = |val: &str, idx: usize, len: usize| {
            counter += 1;
            format!("{}[{}]", val, counter)
        };
        assert_eq!(
            "1[1]2[2]3[3]",
            v.iter().iter_string_mut_rule_ref_exact(&mut rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_rule_ref_exact_basic() {
        let v = [1, 2, 3];
        let state = "key";
        let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}={}", s, val);
        assert_eq!(
            "key=1key=2key=3",
            v.iter()
                .iter_string_with_state_rule_ref_exact(&state, &rule)
        );
    }

    #[test]
    fn test_iterator_string_with_state_mut_rule_ref_exact_basic() {
        let v = [1, 2, 3];
        let mut state = 100;
        let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            **s += 10;
            format!("{}+{}", val, s)
        };
        assert_eq!(
            "1+1102+1203+130",
            v.iter()
                .iter_string_with_state_mut_rule_ref_exact(&mut state, &mut rule)
        );
    }

    // ========================================================================
    // Display Trait Tests
    // ========================================================================
    struct DisplayWrapper<'a, T>(&'a Vec<T>);

    impl<'a, T: core::fmt::Display> core::fmt::Display for DisplayWrapper<'a, T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            DisplayVecString::fmt(self.0, f, DEFAULT_FORMAT_RULE)
        }
    }

    #[test]
    fn test_display_vec_string() {
        let v = vec![1, 2, 3];
        let wrapper = DisplayWrapper(&v);
        assert_eq!("[1, 2, 3]", format!("{}", wrapper));
    }

    struct DisplayFnWrapper<'a, T, F>(&'a Vec<T>, F)
    where
        F: Fn(&str, usize, usize) -> String;

    impl<'a, T: core::fmt::Display, F: Fn(&str, usize, usize) -> String> core::fmt::Display
        for DisplayFnWrapper<'a, T, F>
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            DisplayVecStringFn::fmt(self.0, f, &self.1)
        }
    }

    #[test]
    fn test_display_vec_string_fn() {
        let v = vec![1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| format!("{{{}}}", val);
        let wrapper = DisplayFnWrapper(&v, rule);
        assert_eq!("{1}{2}{3}", format!("{}", wrapper));
    }

    // ========================================================================
    // Rayon Tests (если feature активен)
    // ========================================================================
    #[cfg(feature = "rayon")]
    mod rayon_tests {
        use super::*;
        use rayon::prelude::*;

        #[test]
        fn test_par_iter_string_basic() {
            let v = vec![1, 2, 3];
            assert_eq!(
                "[1, 2, 3]",
                v.par_iter().par_iter_string(DEFAULT_FORMAT_RULE)
            );
        }

        #[test]
        fn test_par_iter_string_fn_basic() {
            let v = vec![1, 2, 3];
            let rule = |val: &str, idx: usize, len: usize| format!("{{{}}}", val);
            assert_eq!("{1}{2}{3}", v.par_iter().par_iter_string_fn(rule));
        }

        #[test]
        fn test_par_iter_string_fn_mut_basic() {
            let v = vec![1, 2, 3];
            let mut counter = 0;
            let rule = |val: &str, idx: usize, len: usize| {
                counter += 1;
                format!("{}#{}", val, counter)
            };
            assert_eq!("1#12#23#3", v.par_iter().par_iter_string_fn_mut(rule));
        }

        #[test]
        fn test_par_iter_string_fn_ptr_basic() {
            fn my_rule(val: &str, idx: usize, len: usize) -> String {
                format!("[{}]", val)
            }
            let v = vec![1, 2, 3];
            assert_eq!("[1][2][3]", v.par_iter().par_iter_string_fn_ptr(my_rule));
        }

        #[test]
        fn test_par_iter_string_with_state_basic() {
            let v = vec![1, 2, 3];
            let mut state = 0;
            let mut i = 0;
            let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
                **s += 1;
                i += 1;
                format!("{}({})", val, s)
            };
            assert_eq!(
                "1(1)2(2)3(3)",
                v.par_iter().par_iter_string_with_state(&mut state, rule)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_fn_basic() {
            let v = vec![1, 2, 3];
            let prefix = "item";
            let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}: {}", s, val);
            assert_eq!(
                "item: 1item: 2item: 3",
                v.par_iter().par_iter_string_with_state_fn(&prefix, rule)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_fn_ptr_basic() {
            fn my_rule(state: &String, val: &str, idx: usize, len: usize) -> String {
                format!("{}={}", state, val)
            }
            let v = vec![1, 2, 3];
            let prefix = "val".to_string();
            assert_eq!(
                "val=1val=2val=3",
                v.par_iter()
                    .par_iter_string_with_state_fn_ptr(&prefix, my_rule)
            );
        }

        #[test]
        fn test_par_iter_string_rule_owned_basic() {
            let v = vec![1, 2, 3];
            let rule = |val: &str, idx: usize, len: usize| format!("[{}]", val);
            assert_eq!("[1][2][3]", v.par_iter().par_iter_string_rule_owned(rule));
        }

        #[test]
        fn test_par_iter_string_mut_rule_owned_basic() {
            let v = vec![1, 2, 3];
            let mut counter = 0;
            let rule = |val: &str, idx: usize, len: usize| {
                counter += 1;
                format!("{}#{}", val, counter)
            };
            assert_eq!(
                "1#12#23#3",
                v.par_iter().par_iter_string_mut_rule_owned(rule)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_rule_owned_basic() {
            let v = vec![1, 2, 3];
            let state = "prefix";
            let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}:{}", s, val);
            assert_eq!(
                "prefix:1prefix:2prefix:3",
                v.par_iter()
                    .par_iter_string_with_state_rule_owned(&state, rule)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_mut_rule_owned_basic() {
            let v = vec![1, 2, 3];
            let mut state = 0;
            let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
                **s += 1;
                format!("{}#{}", val, s)
            };
            assert_eq!(
                "1#12#23#3",
                v.par_iter()
                    .par_iter_string_with_state_mut_rule_owned(&mut state, rule)
            );
        }

        #[test]
        fn test_par_iter_string_rule_ref_basic() {
            let v = vec![1, 2, 3];
            let rule = |val: &str, idx: usize, len: usize| format!("({})", val);
            assert_eq!("(1)(2)(3)", v.par_iter().par_iter_string_rule_ref(&rule));
        }

        #[test]
        fn test_par_iter_string_mut_rule_ref_basic() {
            let v = vec![1, 2, 3];
            let mut counter = 0;
            let mut rule = |val: &str, idx: usize, len: usize| {
                counter += 1;
                format!("{}[{}]", val, counter)
            };
            assert_eq!(
                "1[1]2[2]3[3]",
                v.par_iter().par_iter_string_mut_rule_ref(&mut rule)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_rule_ref_basic() {
            let v = vec![1, 2, 3];
            let state = "key";
            let rule = |s: &&str, val: &str, idx: usize, len: usize| format!("{}={}", s, val);
            assert_eq!(
                "key=1key=2key=3",
                v.par_iter()
                    .par_iter_string_with_state_rule_ref(&state, &rule)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_mut_rule_ref_basic() {
            let v = vec![1, 2, 3];
            let mut state = 100;
            let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
                **s += 10;
                format!("{}+{}", val, s)
            };
            assert_eq!(
                "1+1102+1203+130",
                v.par_iter()
                    .par_iter_string_with_state_mut_rule_ref(&mut state, &mut rule)
            );
        }
    }

    // ========================================================================
    // Async Tests (если feature активен)
    // ========================================================================
    #[cfg(feature = "dyn_async")]
    mod dyn_async_tests {
        use super::*;
        use alloc::boxed::Box;
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
        fn test_vec_string_fn_async_basic() {
            let v = vec![1, 2, 3];
            let rule = |val: &str, idx: usize, len: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let fut = VecStringFnAsync::vec_string_async_fn(&v, &rule);
            let result = block_on_dyn(fut);
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_vec_string_fn_mut_async_basic() {
            let v = vec![1, 2, 3];
            let mut counter = 0;
            let mut rule = |val: &str, idx: usize, len: usize| {
                counter += 1;
                let c = counter;
                let val = val.to_string();
                async move { format!("{}#{}", val, c) }
            };
            let fut = VecStringFnMutImplAsyncSend::vec_string_async_fn_mut(&v, &mut rule);
            let result = block_on_dyn(Box::new(fut));
            assert_eq!("1#12#23#3", result);
        }

        #[test]
        fn test_vec_string_with_state_async_basic() {
            let v = vec![1, 2, 3];
            let mut state = 0;
            let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
                **s += 1;
                let val = val.to_string();
                let current = **s;
                async move { format!("{}({})", val, current) }
            };
            let fut =
                VecStringWithStateImplAsync::vec_string_with_state_async(&v, &mut state, &mut rule);
            let result = block_on_dyn(Box::new(fut));
            assert_eq!("1(1)2(2)3(3)", result);
        }
    }

    #[cfg(feature = "impl_async")]
    mod impl_async_tests {
        use super::*;
        use core::future::Future;

        fn block_on_impl<F: Future>(mut fut: F) -> F::Output {
            let mut fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };
            let waker = noop_raw_waker();
            let mut cx = core::task::Context::from_waker(&waker);
            loop {
                if let core::task::Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                    return val;
                }
                core::hint::spin_loop();
            }
        }

        fn noop_raw_waker() -> core::task::Waker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            let raw = core::task::RawWaker::new(core::ptr::null(), &VTABLE);
            unsafe { core::task::Waker::from_raw(raw) }
        }

        #[test]
        fn test_vec_string_fn_impl_async_basic() {
            let v = vec![1, 2, 3];
            let rule = |val: &str, idx: usize, len: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let fut = VecStringFnImplAsyncSend::vec_string_async_fn(&v, &rule);
            let result = block_on_impl(fut);
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_vec_string_fn_mut_impl_async_basic() {
            let v = vec![1, 2, 3];
            let mut counter = 0;
            let mut rule = |val: &str, idx: usize, len: usize| {
                counter += 1;
                let c = counter;
                let val = val.to_string();
                async move { format!("{}#{}", val, c) }
            };

            let result = block_on_impl(VecStringFnMutImplAsync::vec_string_async_fn_mut(
                &v, &mut rule,
            ));
            assert_eq!("1#12#23#3", result);
        }

        #[test]
        fn test_vec_string_with_state_impl_async_basic() {
            let v = vec![1, 2, 3];
            let mut state = 0;
            let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
                **s += 1;
                let val = val.to_string();
                let current = **s;
                async move { format!("{}({})", val, current) }
            };
            let fut =
                VecStringWithStateImplAsync::vec_string_with_state_async(&v, &mut state, &mut rule);
            let result = block_on_impl(fut);
            assert_eq!("1(1)2(2)3(3)", result);
        }
    }

    // ========================================================================
    // Custom Format Rules Tests
    // ========================================================================
    #[test]
    fn test_custom_format_rule_csv() {
        let v = [1, 2, 3];
        let rule = |val: &str, idx: usize, len: usize| {
            if idx == len - 1 {
                val.to_string()
            } else {
                format!("{}, ", val)
            }
        };
        assert_eq!("1, 2, 3", v.vec_string(rule));
    }

    #[test]
    fn test_custom_format_rule_json_array() {
        let v = ["a", "b", "c"];
        let rule = |val: &str, idx: usize, len: usize| {
            if len == 0 {
                return String::new();
            }
            let is_last = idx == len - 1;
            if idx == 0 {
                if is_last {
                    format!("[\"{}\"]", val)
                } else {
                    format!("[\"{}\", ", val)
                }
            } else if is_last {
                format!("\"{}\"]", val)
            } else {
                format!("\"{}\", ", val)
            }
        };
        assert_eq!("[\"a\", \"b\", \"c\"]", v.vec_string(rule));
    }

    #[test]
    fn test_custom_format_rule_numbered() {
        let v = ["x", "y", "z"];
        let rule = |val: &str, idx: usize, len: usize| format!("{}. {} ", idx + 1, val);
        assert_eq!("1. x 2. y 3. z ", v.vec_string(rule));
    }

    // ========================================================================
    // Iterator Adapter Tests
    // ========================================================================
    #[test]
    fn test_iterator_with_map() {
        let v = [1, 2, 3];
        let result = v.iter().map(|x| x * 2).iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[2, 4, 6]", result);
    }

    #[test]
    fn test_iterator_with_filter() {
        let v = [1, 2, 3, 4, 5, 6];
        let result = v
            .iter()
            .filter(|&&x| x % 2 == 0)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[2, 4, 6]", result);
    }

    #[test]
    fn test_iterator_with_filter_map() {
        let v = [1, 2, 3, 4, 5];
        let result = v
            .iter()
            .filter_map(|&x| if x % 2 == 0 { Some(x) } else { None })
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[2, 4]", result);
    }

    #[test]
    fn test_iterator_with_enumerate() {
        let v = ["a", "b", "c"];
        let result = v
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}:{}", i, s))
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[0:a, 1:b, 2:c]", result);
    }

    #[test]
    fn test_iterator_with_take() {
        let v = [1, 2, 3, 4, 5];
        let result = v.iter().take(3).iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", result);
    }

    #[test]
    fn test_iterator_with_skip() {
        let v = [1, 2, 3, 4, 5];
        let result = v.iter().skip(2).iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[3, 4, 5]", result);
    }

    #[test]
    fn test_iterator_with_chain() {
        let v1 = [1, 2];
        let v2 = [3, 4];
        let result = v1.iter().chain(v2.iter()).iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3, 4]", result);
    }

    #[test]
    fn test_iterator_with_zip() {
        let v1 = [1, 2, 3];
        let v2 = ["a", "b", "c"];
        let result = v1
            .iter()
            .zip(v2.iter())
            .map(|(n, s)| format!("{}:{}", n, s))
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1:a, 2:b, 3:c]", result);
    }

    // ========================================================================
    // IntoIter Tests
    // ========================================================================
    #[test]
    fn test_into_iter() {
        let v = vec![1, 2, 3];
        let result = v.into_iter().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", result);
    }

    #[test]
    fn test_array_into_iter() {
        let arr = [1, 2, 3];
        let result = arr.into_iter().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", result);
    }

    // ========================================================================
    // StableIter Marker Tests
    // ========================================================================
    #[test]
    fn test_stable_iter_slice_iter() {
        let v = [1, 2, 3];
        fn assert_stable<I: StableIter>(_: &I) {}
        assert_stable(&v.iter());
    }

    #[test]
    fn test_stable_iter_vec_into_iter() {
        let v = vec![1, 2, 3];
        fn assert_stable<I: StableIter>(_: &I) {}
        assert_stable(&v.into_iter());
    }

    #[test]
    fn test_stable_iter_map() {
        let v = [1, 2, 3];
        fn assert_stable<I: StableIter>(_: &I) {}
        assert_stable(&v.iter().map(|x| x));
    }

    #[test]
    fn test_stable_iter_filter() {
        let v = [1, 2, 3];
        fn assert_stable<I: StableIter>(_: &I) {}
        assert_stable(&v.iter().filter(|&&x| x > 1));
    }

    // ========================================================================
    // ExtendedDisplay Tests
    // ========================================================================
    #[test]
    fn test_extended_display_vec() {
        let v = vec![1, 2, 3];
        fn assert_extended<T: ExtendedDisplay>(_: &T) {}
        assert_extended(&v);
    }

    #[test]
    fn test_extended_display_iter() {
        let v = [1, 2, 3];
        fn assert_extended<T: ExtendedDisplay>(_: &T) {}
        assert_extended(&v.iter());
    }

    #[test]
    fn test_extended_display_map_iter() {
        let v = [1, 2, 3];
        fn assert_extended<T: ExtendedDisplay>(_: &T) {}
        assert_extended(&v.iter());
    }

    // ========================================================================
    // Format Rule Trait Tests
    // ========================================================================
    #[test]
    fn test_format_rule_no_state() {
        let rule = |val: &str, idx: usize, len: usize| format!("[{}]", val);
        let result = rule.format("test", 0, 1);
        assert_eq!("[test]", result);
    }

    #[test]
    fn test_format_rule_no_state_owned() {
        let rule = |val: &str, idx: usize, len: usize| format!("[{}]", val);
        let result = rule.format("test", 0, 1);
        assert_eq!("[test]", result);
    }

    #[test]
    fn test_format_rule_mut_no_state() {
        let mut counter = 0;
        let mut rule = |val: &&str, idx: usize, len: usize| {
            counter += 1;
            format!("{}#{}", val, counter)
        };
        let str = "test";
        let result = rule(&str, 0, 1);
        assert_eq!("test#1", result);
    }

    #[test]
    fn test_format_rule_with_state() {
        let rule =
            |state: &String, val: &str, idx: usize, len: usize| format!("{}: {}", state, val);
        let state = "prefix".to_string();
        let result = rule.format(&state, "test", 0, 1);
        assert_eq!("prefix: test", result);
    }

    #[test]
    fn test_format_rule_mut_with_state() {
        let mut state = 0;
        let mut rule = |s: &mut i32, val: &str, idx: usize, len: usize| {
            *s += 1;
            format!("{}#{}", val, s)
        };
        let result = rule.format(&mut state, "test", 0, 1);
        assert_eq!("test#1", result);
    }

    // ========================================================================
    // Default Format Rule Tests
    // ========================================================================
    #[test]
    fn test_default_format_rule_empty() {
        assert_eq!("", DEFAULT_FORMAT_RULE("test", 0, 0));
    }

    #[test]
    fn test_default_format_rule_single() {
        assert_eq!("[test]", DEFAULT_FORMAT_RULE("test", 0, 1));
    }

    #[test]
    fn test_default_format_rule_first() {
        assert_eq!("[test", DEFAULT_FORMAT_RULE("test", 0, 3));
    }

    #[test]
    fn test_default_format_rule_middle() {
        assert_eq!(", test", DEFAULT_FORMAT_RULE("test", 1, 3));
    }

    #[test]
    fn test_default_format_rule_last() {
        assert_eq!(", test]", DEFAULT_FORMAT_RULE("test", 2, 3));
    }

    // ========================================================================
    // Complex Scenario Tests
    // ========================================================================
    #[test]
    fn test_nested_format_rules() {
        let v = [vec![1, 2], vec![3, 4]];
        let inner_rule = |val: &str, idx: usize, len: usize| {
            if idx == len - 1 {
                val.to_string()
            } else {
                format!("{}, ", val)
            }
        };
        let outer_rule = |val: &str, idx: usize, len: usize| {
            if idx == len - 1 {
                val.to_string()
            } else {
                format!("{}, ", val)
            }
        };

        let result: Vec<String> = v.iter().map(|inner| inner.vec_string(inner_rule)).collect();

        let final_result = result.iter().map(|s| s.as_str()).iter_string(outer_rule);
        assert_eq!("1, 2, 3, 4", final_result);
    }

    #[test]
    fn test_chained_operations() {
        let v = [1, 2, 3, 4, 5, 6];
        let result = v
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 10)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[20, 40, 60]", result);
    }

    #[test]
    fn test_multiple_stateful_operations() {
        let v = [1, 2, 3];
        let mut sum = 0;
        let rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
            let num: i32 = val.parse().unwrap_or(0);
            **s += num;
            if idx == len - 1 {
                format!("{} (sum={})", val, s)
            } else {
                format!("{}, ", val)
            }
        };
        let result = v.iter().iter_string_with_state(&mut sum, rule);
        assert_eq!("1, 2, 3 (sum=6)", result);
        assert_eq!(6, sum);
    }
}

// ========================================================================
// Coverage Boosting Tests (Добавить в конец файла)
// ========================================================================
#[cfg(test)]
mod coverage_tests {
    use super::*;
    use alloc::vec;

    // --- Тесты для непокрытых адаптеров StableIter ---
    #[test]
    fn test_stable_iter_iter_mut() {
        let mut v = [1, 2, 3];
        fn assert_stable<I: StableIter>(_: &I) {}
        assert_stable(&v.iter_mut());
        let res = v.iter_mut().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[test]
    fn test_stable_iter_take_while() {
        let v = [1, 2, 3, 4, 5];
        let res = v
            .iter()
            .take_while(|&&x| x < 4)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[test]
    fn test_stable_iter_skip_while() {
        let v = [1, 2, 3, 4, 5];
        let res = v
            .iter()
            .skip_while(|&&x| x < 3)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[3, 4, 5]", res);
    }

    #[test]
    fn test_stable_iter_cloned() {
        let v = [1, 2, 3];
        let res = v.iter().cloned().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[test]
    fn test_stable_iter_copied() {
        let v = [1, 2, 3];
        let res = v.iter().copied().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[test]
    fn test_stable_iter_flat_map() {
        let v = [1, 2];
        let res = v
            .iter()
            .flat_map(|&x| [x, x * 10].into_iter())
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 10, 2, 20]", res);
    }

    #[test]
    fn test_stable_iter_flatten() {
        let v = [[1, 2], [3, 4]];
        let res = v.iter().flatten().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3, 4]", res);
    }

    #[test]
    fn test_stable_iter_fuse() {
        let v = [1, 2, 3];
        let res = v.iter().fuse().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[test]
    fn test_stable_iter_peekable() {
        let v = [1, 2, 3];
        let res = v.iter().peekable().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[test]
    fn test_stable_iter_step_by() {
        let v = [1, 2, 3, 4, 5];
        let res = v.iter().step_by(2).iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 3, 5]", res);
    }

    #[test]
    fn test_stable_iter_cycle() {
        let v = [1, 2];
        let res = v.iter().cycle().take(5).iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 1, 2, 1]", res);
    }

    #[test]
    fn test_stable_iter_rev() {
        let v = [1, 2, 3];
        let res = v.iter().rev().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[3, 2, 1]", res);
    }

    #[test]
    fn test_stable_iter_once() {
        let res = core::iter::once(42).iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[42]", res);
    }

    #[test]
    fn test_stable_iter_empty() {
        let res = core::iter::empty::<i32>().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("", res);
    }

    #[test]
    fn test_stable_iter_repeat() {
        let res = core::iter::repeat(7)
            .take(3)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[7, 7, 7]", res);
    }

    #[test]
    fn test_stable_iter_repeat_with() {
        let mut count = 0;
        let items: Vec<i32> = core::iter::repeat_with(|| {
            count += 1;
            count
        })
        .take(3)
        .collect();
        let res = items.into_iter().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[test]
    fn test_stable_iter_inspect() {
        let v = [1, 2, 3];
        let mut sum = 0;
        let res = v
            .iter()
            .inspect(|&&x| sum += x)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
        assert_eq!(6, sum);
    }

    #[test]
    fn test_stable_iter_scan() {
        let v = [1, 2, 3];
        let res = v
            .iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 3, 6]", res);
    }

    // --- Покрытие для dyn_async итераторов ---
    #[cfg(feature = "dyn_async")]
    mod dyn_async_coverage {
        use super::*;
        use alloc::boxed::Box;
        use core::future::Future;

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

        fn noop_raw_waker() -> core::task::RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            core::task::RawWaker::new(core::ptr::null(), &VTABLE)
        }

        #[test]
        fn test_dyn_async_iter_fn_send() {
            let v = vec![1, 2, 3];
            let rule = |val: &str, idx: usize, len: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let fut = IteratorStringFnAsyncSend::iter_string_async_fn(v.iter(), &rule);
            let result = block_on_dyn(fut);
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_dyn_async_iter_fn_mut() {
            let v = vec![1, 2, 3];
            let mut counter = 0;
            let mut rule = |val: &str, idx: usize, len: usize| {
                counter += 1;
                let c = counter;
                let val = val.to_string();
                async move { format!("{}#{}", val, c) }
            };
            let fut = IteratorStringFnMutAsync::iter_string_async_fn_mut(v.iter(), &mut rule);
            let result = block_on_dyn(fut);
            assert_eq!("1#12#23#3", result);
        }

        #[test]
        fn test_dyn_async_iter_with_state() {
            let v = vec![1, 2, 3];
            let mut state = 0;
            let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
                **s += 1;
                let val = val.to_string();
                let current = **s;
                async move { format!("{}({})", val, current) }
            };
            let fut = IteratorStringWithStateAsync::iter_string_with_state_async(
                v.iter(),
                &mut state,
                &mut rule,
            );
            let result = block_on_dyn(fut);
            assert_eq!("1(1)2(2)3(3)", result);
        }
    }

    // --- Покрытие для impl_async итераторов ---
    #[cfg(feature = "impl_async")]
    mod impl_async_coverage {
        use super::*;
        use core::future::Future;

        fn block_on_impl<F: Future>(mut fut: F) -> F::Output {
            let mut fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };
            let waker = noop_raw_waker();
            let mut cx = core::task::Context::from_waker(&waker);
            loop {
                if let core::task::Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                    return val;
                }
                core::hint::spin_loop();
            }
        }

        fn noop_raw_waker() -> core::task::Waker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            let raw = core::task::RawWaker::new(core::ptr::null(), &VTABLE);
            unsafe { core::task::Waker::from_raw(raw) }
        }

        #[test]
        fn test_impl_async_iter_fn_send() {
            let v = vec![1, 2, 3];
            let rule = |val: &str, idx: usize, len: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let fut = IteratorStringFnImplAsyncSend::iter_string_async_fn(v.iter(), &rule);
            let result = block_on_impl(fut);
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_impl_async_iter_fn_mut() {
            let v = vec![1, 2, 3];
            let mut counter = 0;
            let mut rule = |val: &str, idx: usize, len: usize| {
                counter += 1;
                let c = counter;
                let val = val.to_string();
                async move { format!("{}#{}", val, c) }
            };
            let result = block_on_impl(IteratorStringFnMutImplAsync::iter_string_async_fn_mut(
                v.iter(),
                &mut rule,
            ));
            assert_eq!("1#12#23#3", result);
        }

        #[test]
        fn test_impl_async_iter_with_state() {
            let v = vec![1, 2, 3];
            let mut state = 0;
            let mut rule = |s: &mut &mut i32, val: &str, idx: usize, len: usize| {
                **s += 1;
                let val = val.to_string();
                let current = **s;
                async move { format!("{}({})", val, current) }
            };
            let result = block_on_impl(
                IteratorStringWithStateImplAsync::iter_string_with_state_async(
                    v.iter(),
                    &mut state,
                    &mut rule,
                ),
            );
            assert_eq!("1(1)2(2)3(3)", result);
        }
    }

    // --- Покрытие для Rayon + dyn_async ---
    #[cfg(all(feature = "rayon", feature = "dyn_async"))]
    mod rayon_dyn_async_coverage {
        use super::*;
        use alloc::boxed::Box;
        use core::future::Future;
        use rayon::prelude::*;

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

        fn noop_raw_waker() -> core::task::RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            core::task::RawWaker::new(core::ptr::null(), &VTABLE)
        }

        #[test]
        fn test_rayon_dyn_async_fn_ptr() {
            let v = vec![1, 2, 3];
            fn my_fmt(val: &str, idx: usize, len: usize) -> String {
                let val = val.to_string();
                format!("[{}]", val)
            }
            let result = block_on_dyn(ParIteratorStringFnPtrAsync::par_iter_string_async_fn_ptr(
                v.par_iter(),
                my_fmt,
            ));
            assert_eq!("[1][2][3]", result);
        }
    }

    // --- Покрытие для Rayon + impl_async ---
    #[cfg(all(feature = "rayon", feature = "impl_async"))]
    mod rayon_impl_async_coverage {
        use super::*;
        use core::future::Future;
        use rayon::prelude::*;

        fn block_on_impl<F: Future>(mut fut: F) -> F::Output {
            let mut fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };
            let waker = noop_raw_waker();
            let mut cx = core::task::Context::from_waker(&waker);
            loop {
                if let core::task::Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                    return val;
                }
                core::hint::spin_loop();
            }
        }

        fn noop_raw_waker() -> core::task::Waker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            let raw = core::task::RawWaker::new(core::ptr::null(), &VTABLE);
            unsafe { core::task::Waker::from_raw(raw) }
        }

        #[test]
        fn test_rayon_impl_async_fn() {
            let v = vec![1, 2, 3];
            let fmt = |value: &str, index: usize, length: usize| {
                let value = value.to_string();
                async move { format!("[{}]", value) }
            };
            let result = block_on_impl(ParIteratorStringFnImplAsync::par_iter_string_async_fn(
                v.par_iter(),
                &fmt,
            ));
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_rayon_impl_async_fn_send() {
            let v = vec![1, 2, 3];
            let fmt = |value: &str, index: usize, length: usize| {
                let value = value.to_string();
                async move { format!("[{}]", value) }
            };
            let result = block_on_impl(ParIteratorStringFnImplAsyncSend::par_iter_string_async_fn(
                v.par_iter(),
                &fmt,
            ));
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_rayon_impl_async_fn_ptr() {
            let v = vec![1, 2, 3];
            fn my_fmt(val: &str, _idx: usize, _len: usize) -> String {
                let val = val.to_string();
                format!("[{}]", val)
            }
            let result = block_on_impl(
                ParIteratorStringFnPtrImplAsync::par_iter_string_async_fn_ptr(v.par_iter(), my_fmt),
            );
            assert_eq!("[1][2][3]", result);
        }
    }
}

#[cfg(test)]
mod nested_tests {
    use super::*;
    use alloc::format;
    use alloc::string::String;
    use alloc::vec;

    // ========================================================================
    // 1. VEC NESTED BASE TRAITS
    // ========================================================================
    #[test]
    fn test_vec_string_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_vec_string_fn_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_fn_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_vec_string_fn_mut_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_fn_mut_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_vec_string_nested_triple() {
        // Triple nesting: inner vecs formatted via VecString, then outer via VecStringNested
        let inner: Vec<Vec<i32>> = vec![vec![1, 2], vec![3]];
        let outer: Vec<Vec<i32>> = vec![vec![4]];
        // Format each group of vecs using nested, then combine
        let r1 = inner.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE);
        let r2 = outer.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE);
        assert_eq!("[[1, 2], [3]]", r1);
        assert_eq!("[[4]]", r2);
    }

    #[test]
    fn test_vec_string_nested_slice() {
        let inner1 = vec![1, 2];
        let inner2 = vec![3, 4];
        let arr = [inner1, inner2];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            arr.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    // ========================================================================
    // 2. VEC NESTED WITH STATE
    // ========================================================================
    #[test]
    fn test_vec_string_with_state_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.vec_string_with_state_nested(DEFAULT_FORMAT_RULE, 0i32, |s, val, i, l| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        });
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    #[test]
    fn test_vec_string_with_state_fn_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.vec_string_with_state_fn_nested(DEFAULT_FORMAT_RULE, &0i32, |_s, val, i, l| {
            DEFAULT_FORMAT_RULE(val, i, l)
        });
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    #[test]
    fn test_vec_string_with_state_fn_ptr_nested() {
        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.vec_string_with_state_fn_ptr_nested(DEFAULT_FORMAT_RULE, &0i32, ptr_rule);
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    // ========================================================================
    // 3. VEC NESTED RULE OWNED
    // ========================================================================
    #[test]
    fn test_vec_string_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_rule_owned_nested(DEFAULT_FORMAT_RULE, r)
        );
    }

    #[test]
    fn test_vec_string_mut_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_mut_rule_owned_nested(DEFAULT_FORMAT_RULE, &mut r)
        );
    }

    #[test]
    fn test_vec_string_with_state_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_with_state_rule_owned_nested(DEFAULT_FORMAT_RULE, &0i32, r)
        );
    }

    #[test]
    fn test_vec_string_with_state_mut_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_with_state_mut_rule_owned_nested(DEFAULT_FORMAT_RULE, 0i32, &mut r)
        );
    }

    // ========================================================================
    // 4. VEC NESTED RULE REF
    // ========================================================================
    #[test]
    fn test_vec_string_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_rule_ref_nested(DEFAULT_FORMAT_RULE, &r)
        );
    }

    #[test]
    fn test_vec_string_mut_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_mut_rule_ref_nested(DEFAULT_FORMAT_RULE, &mut r)
        );
    }

    #[test]
    fn test_vec_string_with_state_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_with_state_rule_ref_nested(DEFAULT_FORMAT_RULE, &0i32, &r)
        );
    }

    #[test]
    fn test_vec_string_with_state_mut_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.vec_string_with_state_mut_rule_ref_nested(DEFAULT_FORMAT_RULE, 0i32, &mut r)
        );
    }

    // ========================================================================
    // 5. ITERATOR NESTED BASE TRAITS
    // ========================================================================
    #[test]
    fn test_iter_string_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iter_string_fn_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_fn_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iter_string_fn_mut_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_fn_mut_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    // ========================================================================
    // 6. ITERATOR NESTED WITH STATE
    // ========================================================================
    #[test]
    fn test_iter_string_with_state_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.into_iter().iter_string_with_state_nested(
            DEFAULT_FORMAT_RULE,
            0i32,
            |s, val, i, l| {
                *s += 1;
                DEFAULT_FORMAT_RULE(val, i, l)
            },
        );
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    #[test]
    fn test_iter_string_with_state_fn_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.into_iter().iter_string_with_state_fn_nested(
            DEFAULT_FORMAT_RULE,
            &0i32,
            |_s, val, i, l| DEFAULT_FORMAT_RULE(val, i, l),
        );
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    #[test]
    fn test_iter_string_with_state_fn_ptr_nested() {
        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.into_iter().iter_string_with_state_fn_ptr_nested(
            DEFAULT_FORMAT_RULE,
            &0i32,
            ptr_rule,
        );
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    // ========================================================================
    // 7. ITERATOR NESTED RULE OWNED
    // ========================================================================
    #[test]
    fn test_iter_string_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_rule_owned_nested(DEFAULT_FORMAT_RULE, r)
        );
    }

    #[test]
    fn test_iter_string_mut_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_mut_rule_owned_nested(DEFAULT_FORMAT_RULE, &mut r)
        );
    }

    #[test]
    fn test_iter_string_with_state_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_with_state_rule_owned_nested(DEFAULT_FORMAT_RULE, &0i32, r)
        );
    }

    #[test]
    fn test_iter_string_with_state_mut_rule_owned_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter().iter_string_with_state_mut_rule_owned_nested(
                DEFAULT_FORMAT_RULE,
                0i32,
                &mut r
            )
        );
    }

    // ========================================================================
    // 8. ITERATOR NESTED RULE REF
    // ========================================================================
    #[test]
    fn test_iter_string_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_rule_ref_nested(DEFAULT_FORMAT_RULE, &r)
        );
    }

    #[test]
    fn test_iter_string_mut_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_mut_rule_ref_nested(DEFAULT_FORMAT_RULE, &mut r)
        );
    }

    #[test]
    fn test_iter_string_with_state_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_with_state_rule_ref_nested(DEFAULT_FORMAT_RULE, &0i32, &r)
        );
    }

    #[test]
    fn test_iter_string_with_state_mut_rule_ref_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter().iter_string_with_state_mut_rule_ref_nested(
                DEFAULT_FORMAT_RULE,
                0i32,
                &mut r
            )
        );
    }

    // ========================================================================
    // 9. EXACT NESTED TRAITS
    // ========================================================================
    #[test]
    fn test_iter_string_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_exact_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iter_string_fn_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_fn_exact_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iter_string_fn_mut_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_fn_mut_exact_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_iter_string_with_state_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.into_iter().iter_string_with_state_exact_nested(
            DEFAULT_FORMAT_RULE,
            0i32,
            |s, val, i, l| {
                *s += 1;
                DEFAULT_FORMAT_RULE(val, i, l)
            },
        );
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    #[test]
    fn test_iter_string_with_state_fn_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.into_iter().iter_string_with_state_fn_exact_nested(
            DEFAULT_FORMAT_RULE,
            &0i32,
            |_s, val, i, l| DEFAULT_FORMAT_RULE(val, i, l),
        );
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    #[test]
    fn test_iter_string_with_state_fn_ptr_exact_nested() {
        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        let v = vec![vec![1, 2], vec![3, 4]];
        let res = v.into_iter().iter_string_with_state_fn_ptr_exact_nested(
            DEFAULT_FORMAT_RULE,
            &0i32,
            ptr_rule,
        );
        assert_eq!("[[1, 2], [3, 4]]", res);
    }

    #[test]
    fn test_iter_string_rule_owned_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_rule_owned_exact_nested(DEFAULT_FORMAT_RULE, r)
        );
    }

    #[test]
    fn test_iter_string_mut_rule_owned_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_mut_rule_owned_exact_nested(DEFAULT_FORMAT_RULE, &mut r)
        );
    }

    #[test]
    fn test_iter_string_with_state_rule_owned_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_with_state_rule_owned_exact_nested(DEFAULT_FORMAT_RULE, &0i32, r)
        );
    }

    #[test]
    fn test_iter_string_with_state_mut_rule_owned_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_with_state_mut_rule_owned_exact_nested(
                    DEFAULT_FORMAT_RULE,
                    0i32,
                    &mut r
                )
        );
    }

    #[test]
    fn test_iter_string_rule_ref_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_rule_ref_exact_nested(DEFAULT_FORMAT_RULE, &r)
        );
    }

    #[test]
    fn test_iter_string_mut_rule_ref_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_mut_rule_ref_exact_nested(DEFAULT_FORMAT_RULE, &mut r)
        );
    }

    #[test]
    fn test_iter_string_with_state_rule_ref_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter().iter_string_with_state_rule_ref_exact_nested(
                DEFAULT_FORMAT_RULE,
                &0i32,
                &r
            )
        );
    }

    #[test]
    fn test_iter_string_with_state_mut_rule_ref_exact_nested() {
        let v = vec![vec![1, 2], vec![3, 4]];
        let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_iter()
                .iter_string_with_state_mut_rule_ref_exact_nested(
                    DEFAULT_FORMAT_RULE,
                    0i32,
                    &mut r
                )
        );
    }

    // ========================================================================
    // 10. RAYON PAR NESTED TRAITS
    // ========================================================================
    #[cfg(feature = "rayon")]
    mod par_nested {
        use super::*;
        use rayon::prelude::*;

        #[test]
        fn test_par_iter_string_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
            );
        }

        #[test]
        fn test_par_iter_string_fn_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_fn_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
            );
        }

        #[test]
        fn test_par_iter_string_fn_mut_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_fn_mut_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
            );
        }

        #[test]
        fn test_par_iter_string_fn_ptr_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_fn_ptr_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let res = v.into_par_iter().par_iter_string_with_state_nested(
                DEFAULT_FORMAT_RULE,
                0i32,
                |s, val, i, l| {
                    *s += 1;
                    DEFAULT_FORMAT_RULE(val, i, l)
                },
            );
            assert_eq!("[[1, 2], [3, 4]]", res);
        }

        #[test]
        fn test_par_iter_string_with_state_fn_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let res = v.into_par_iter().par_iter_string_with_state_fn_nested(
                DEFAULT_FORMAT_RULE,
                &0i32,
                |_s, val, i, l| DEFAULT_FORMAT_RULE(val, i, l),
            );
            assert_eq!("[[1, 2], [3, 4]]", res);
        }

        #[test]
        fn test_par_iter_string_with_state_fn_ptr_nested() {
            fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
                DEFAULT_FORMAT_RULE(v, i, l)
            }
            let v = vec![vec![1, 2], vec![3, 4]];
            let res = v.into_par_iter().par_iter_string_with_state_fn_ptr_nested(
                DEFAULT_FORMAT_RULE,
                &0i32,
                ptr_rule,
            );
            assert_eq!("[[1, 2], [3, 4]]", res);
        }

        #[test]
        fn test_par_iter_string_rule_owned_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_rule_owned_nested(DEFAULT_FORMAT_RULE, r)
            );
        }

        #[test]
        fn test_par_iter_string_mut_rule_owned_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_mut_rule_owned_nested(DEFAULT_FORMAT_RULE, &mut r)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_rule_owned_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_with_state_rule_owned_nested(DEFAULT_FORMAT_RULE, &0i32, r)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_mut_rule_owned_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
                *s += 1;
                DEFAULT_FORMAT_RULE(val, i, l)
            };
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_with_state_mut_rule_owned_nested(
                        DEFAULT_FORMAT_RULE,
                        0i32,
                        &mut r
                    )
            );
        }

        #[test]
        fn test_par_iter_string_rule_ref_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_rule_ref_nested(DEFAULT_FORMAT_RULE, &r)
            );
        }

        #[test]
        fn test_par_iter_string_mut_rule_ref_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let mut r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_mut_rule_ref_nested(DEFAULT_FORMAT_RULE, &mut r)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_rule_ref_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let r = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_with_state_rule_ref_nested(DEFAULT_FORMAT_RULE, &0i32, &r)
            );
        }

        #[test]
        fn test_par_iter_string_with_state_mut_rule_ref_nested() {
            let v = vec![vec![1, 2], vec![3, 4]];
            let mut r = |s: &mut i32, val: &str, i: usize, l: usize| {
                *s += 1;
                DEFAULT_FORMAT_RULE(val, i, l)
            };
            assert_eq!(
                "[[1, 2], [3, 4]]",
                v.into_par_iter()
                    .par_iter_string_with_state_mut_rule_ref_nested(
                        DEFAULT_FORMAT_RULE,
                        0i32,
                        &mut r
                    )
            );
        }
    }

    // ========================================================================
    // 10d. ITERTOOLS TRAITS
    // ========================================================================
    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_interleave() {
        use itertools::Itertools;
        let a = vec![1, 3];
        let b = vec![2, 4];
        let res = a
            .iter()
            .interleave(b.iter())
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3, 4]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_interleave_shortest() {
        use itertools::Itertools;
        let a = vec![1, 3, 5];
        let b = vec![2, 4];
        let res = a
            .iter()
            .interleave_shortest(b.iter())
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3, 4, 5]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_unique() {
        use itertools::Itertools;
        let v = vec![1, 2, 2, 3, 1];
        let res = v.iter().unique().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_pad_using() {
        use itertools::Itertools;
        let v = vec![1, 2];
        let res = v
            .iter()
            .copied()
            .pad_using(4, |i| i * 10)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 20, 30]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_positions() {
        use itertools::Itertools;
        let v = vec![1, 2, 3, 4];
        let res = v
            .iter()
            .positions(|&x| x % 2 == 0)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 3]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_update() {
        use itertools::Itertools;
        let v = vec![1, 2, 3];
        let res = v
            .into_iter()
            .update(|x| *x *= 2)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[2, 4, 6]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_while_some() {
        use itertools::Itertools;
        let v = vec![Some(1), Some(2), None, Some(3)];
        let res = v.into_iter().while_some().iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_batching() {
        use itertools::Itertools;
        let v = vec![1, 2, 3, 4, 5];
        let res = v
            .into_iter()
            .batching(|it| it.next())
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3, 4, 5]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_put_back() {
        use itertools::put_back;
        let v = vec![1, 2, 3];
        let mut it = put_back(v.into_iter());
        it.next();
        it.put_back(1);
        let res = it.iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_map_into() {
        use itertools::Itertools;
        let v = vec![1i32, 2, 3];
        let res = v
            .into_iter()
            .map_into::<i64>()
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_take_while_inclusive() {
        use itertools::Itertools;
        let v = vec![1, 2, 3, 4, 5];
        let res = v
            .into_iter()
            .take_while_inclusive(|&x| x < 4)
            .iter_string(DEFAULT_FORMAT_RULE);
        assert_eq!("[1, 2, 3, 4]", res);
    }

    #[cfg(feature = "itertools")]
    #[test]
    fn test_itertools_stable_iter_non_display() {
        use itertools::Itertools;
        fn assert_stable<I: StableIter>(_: &I) {}
        let v = vec![1, 2, 3];
        // combinations yields Vec<&i32> - no Display, but StableIter
        assert_stable(&v.iter().combinations(2));
        // permutations yields Vec<&i32>
        assert_stable(&v.iter().permutations(2));
        // cartesian_product yields (&i32, &i32)
        let a = vec![1, 2];
        let b = vec![3, 4];
        assert_stable(&a.iter().cartesian_product(b.iter()));
        // zip_eq yields (&i32, &i32)
        assert_stable(&a.iter().zip_eq(b.iter()));
        // zip_longest yields EitherOrBoth
        assert_stable(&a.iter().zip_longest(b.iter()));
        // with_position yields Position<&i32>
        assert_stable(&v.iter().with_position());
        // powerset yields Vec<&i32>
        assert_stable(&v.iter().powerset());
        // combinations_with_replacement yields Vec<&i32>
        assert_stable(&v.iter().combinations_with_replacement(2));
        // multi_cartesian_product yields Vec<i32>
        let vv = vec![vec![1, 2], vec![3, 4]];
        assert_stable(&vv.into_iter().multi_cartesian_product());
        // filter_ok yields Result<i32, &str>
        let vr: Vec<Result<i32, &str>> = vec![Ok(1), Err("e")];
        assert_stable(&vr.into_iter().filter_ok(|&x| x > 0));
    }

    // ========================================================================
    // 10e. ITERMORE TRAITS
    // ========================================================================
    #[cfg(feature = "itermore")]
    #[test]
    fn test_itermore_stable_iter() {
        use itermore::prelude::*;
        fn assert_stable<I: StableIter>(_: &I) {}
        let v = vec![1, 2, 3, 4];
        // array_chunks yields [i32; 2]
        assert_stable(&v.clone().into_iter().array_chunks::<2>());
        // array_windows yields [i32; 2]
        assert_stable(&v.clone().into_iter().array_windows::<2>());
        // array_combinations yields [i32; 2]
        assert_stable(&v.clone().into_iter().array_combinations::<2>());
        // array_combinations_with_reps yields [i32; 2]
        assert_stable(&v.clone().into_iter().array_combinations_with_reps::<2>());
        // cartesian_product yields (i32, i32)
        let a = vec![1, 2];
        let b = vec![3, 4];
        assert_stable(&a.clone().into_iter().cartesian_product(b));
        // circular_array_windows yields [i32; 2]
        assert_stable(&v.clone().into_iter().circular_array_windows::<2>());
        // combinations yields Vec<i32>
        assert_stable(&v.clone().into_iter().combinations(2));
        // combinations_with_reps yields Vec<i32>
        assert_stable(&v.into_iter().combinations_with_reps(2));
    }

    // ========================================================================
    // 10b. ORX-PARALLEL TRAITS
    // ========================================================================
    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_base_traits() {
        use orx_parallel::*;
        let v = vec![1, 2, 3];
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_fn(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_fn_mut(DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[1, 2, 3]",
            v.into_par().orx_par_iter_string_fn_ptr(DEFAULT_FORMAT_RULE)
        );
    }

    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_with_state_traits() {
        use orx_parallel::*;
        let v = vec![1, 2, 3];
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_with_state(0i32, |s, val, i, l| {
                    *s += 1;
                    DEFAULT_FORMAT_RULE(val, i, l)
                })
        );
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_with_state_fn(&0i32, |_s, val, i, l| {
                    DEFAULT_FORMAT_RULE(val, i, l)
                })
        );
        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        assert_eq!(
            "[1, 2, 3]",
            v.into_par()
                .orx_par_iter_string_with_state_fn_ptr(&0i32, ptr_rule)
        );
    }

    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_rule_owned_traits() {
        use orx_parallel::*;
        let v = vec![1, 2, 3];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.clone().into_par().orx_par_iter_string_rule_owned(r)
        );
        let mut mr = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_mut_rule_owned(&mut mr)
        );
        let sr = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_with_state_rule_owned(&0i32, sr)
        );
        let mut smr = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[1, 2, 3]",
            v.into_par()
                .orx_par_iter_string_with_state_mut_rule_owned(0i32, &mut smr)
        );
    }

    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_rule_ref_traits() {
        use orx_parallel::*;
        let v = vec![1, 2, 3];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.clone().into_par().orx_par_iter_string_rule_ref(&r)
        );
        let mut mr = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_mut_rule_ref(&mut mr)
        );
        let sr = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[1, 2, 3]",
            v.clone()
                .into_par()
                .orx_par_iter_string_with_state_rule_ref(&0i32, &sr)
        );
        let mut smr = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[1, 2, 3]",
            v.into_par()
                .orx_par_iter_string_with_state_mut_rule_ref(0i32, &mut smr)
        );
    }

    // ========================================================================
    // 10c. ORX-PARALLEL NESTED TRAITS
    // ========================================================================
    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_nested_base_traits() {
        use orx_parallel::*;
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_fn_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_fn_mut_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_par()
                .orx_par_iter_string_fn_ptr_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_nested_with_state_traits() {
        use orx_parallel::*;
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone().into_par().orx_par_iter_string_with_state_nested(
                DEFAULT_FORMAT_RULE,
                0i32,
                |s, val, i, l| {
                    *s += 1;
                    DEFAULT_FORMAT_RULE(val, i, l)
                }
            )
        );
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_with_state_fn_nested(
                    DEFAULT_FORMAT_RULE,
                    &0i32,
                    |_s, val, i, l| { DEFAULT_FORMAT_RULE(val, i, l) }
                )
        );
        fn ptr_rule(_s: &i32, v: &str, i: usize, l: usize) -> String {
            DEFAULT_FORMAT_RULE(v, i, l)
        }
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_par().orx_par_iter_string_with_state_fn_ptr_nested(
                DEFAULT_FORMAT_RULE,
                &0i32,
                ptr_rule
            )
        );
    }

    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_nested_rule_owned_traits() {
        use orx_parallel::*;
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_rule_owned_nested(DEFAULT_FORMAT_RULE, r)
        );
        let mut mr = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_mut_rule_owned_nested(DEFAULT_FORMAT_RULE, &mut mr)
        );
        let sr = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_with_state_rule_owned_nested(DEFAULT_FORMAT_RULE, &0i32, sr)
        );
        let mut smr = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_par()
                .orx_par_iter_string_with_state_mut_rule_owned_nested(
                    DEFAULT_FORMAT_RULE,
                    0i32,
                    &mut smr
                )
        );
    }

    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_nested_rule_ref_traits() {
        use orx_parallel::*;
        let v = vec![vec![1, 2], vec![3, 4]];
        let r = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_rule_ref_nested(DEFAULT_FORMAT_RULE, &r)
        );
        let mut mr = |val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_mut_rule_ref_nested(DEFAULT_FORMAT_RULE, &mut mr)
        );
        let sr = |_s: &i32, val: &str, i: usize, l: usize| DEFAULT_FORMAT_RULE(val, i, l);
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.clone()
                .into_par()
                .orx_par_iter_string_with_state_rule_ref_nested(DEFAULT_FORMAT_RULE, &0i32, &sr)
        );
        let mut smr = |s: &mut i32, val: &str, i: usize, l: usize| {
            *s += 1;
            DEFAULT_FORMAT_RULE(val, i, l)
        };
        assert_eq!(
            "[[1, 2], [3, 4]]",
            v.into_par()
                .orx_par_iter_string_with_state_mut_rule_ref_nested(
                    DEFAULT_FORMAT_RULE,
                    0i32,
                    &mut smr
                )
        );
    }

    // ========================================================================
    // 10b. ORX-PARALLEL + DYN ASYNC TRAITS
    // ========================================================================
    #[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
    mod orx_dyn_async_coverage {
        use super::*;
        use alloc::boxed::Box;
        use core::future::Future;
        use orx_parallel::*;

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

        fn noop_raw_waker() -> core::task::RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            core::task::RawWaker::new(core::ptr::null(), &VTABLE)
        }

        #[test]
        fn test_orx_dyn_async_fn_ptr() {
            let v = vec![1, 2, 3];
            fn my_fmt(val: &str, _idx: usize, _len: usize) -> String {
                format!("[{}]", val)
            }
            let result = block_on_dyn(
                OrxParIteratorStringFnPtrAsync::orx_par_iter_string_async_fn_ptr(
                    v.into_par(),
                    my_fmt,
                ),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_dyn_async_fn() {
            let v = vec![1, 2, 3];
            let fmt = |value: &str, _index: usize, _length: usize| {
                let value = value.to_string();
                async move { format!("[{}]", value) }
            };
            let result = block_on_dyn(OrxParIteratorStringFnAsync::orx_par_iter_string_async_fn(
                v.into_par(),
                &fmt,
            ));
            assert_eq!("[1][2][3]", result);
        }
    }

    // ========================================================================
    // 10c. ORX-PARALLEL + IMPL ASYNC TRAITS
    // ========================================================================
    #[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
    mod orx_impl_async_coverage {
        use super::*;
        use core::future::Future;
        use orx_parallel::*;

        fn block_on_impl<F: Future>(mut fut: F) -> F::Output {
            let mut fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };
            let waker = noop_raw_waker();
            let mut cx = core::task::Context::from_waker(&waker);
            loop {
                if let core::task::Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                    return val;
                }
                core::hint::spin_loop();
            }
        }

        fn noop_raw_waker() -> core::task::Waker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            let raw = core::task::RawWaker::new(core::ptr::null(), &VTABLE);
            unsafe { core::task::Waker::from_raw(raw) }
        }

        #[test]
        fn test_orx_impl_async_fn() {
            let v = vec![1, 2, 3];
            let fmt = |value: &str, _index: usize, _length: usize| {
                let value = value.to_string();
                async move { format!("[{}]", value) }
            };
            let result = block_on_impl(
                OrxParIteratorStringFnImplAsync::orx_par_iter_string_async_fn(v.into_par(), &fmt),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_impl_async_fn_send() {
            let v = vec![1, 2, 3];
            let fmt = |value: &str, _index: usize, _length: usize| {
                let value = value.to_string();
                async move { format!("[{}]", value) }
            };
            let result = block_on_impl(
                OrxParIteratorStringFnImplAsyncSend::orx_par_iter_string_async_fn(
                    v.into_par(),
                    &fmt,
                ),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_impl_async_fn_ptr() {
            let v = vec![1, 2, 3];
            fn my_fmt(val: &str, _idx: usize, _len: usize) -> String {
                format!("[{}]", val)
            }
            let result = block_on_impl(
                OrxParIteratorStringFnPtrImplAsync::orx_par_iter_string_async_fn_ptr(
                    v.into_par(),
                    my_fmt,
                ),
            );
            assert_eq!("[1][2][3]", result);
        }
    }

    // ========================================================================
    // 10c. ORX CLONE TRAITS
    // ========================================================================
    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_clone_traits() {
        use orx_parallel::*;
        let f = |val: &str, i: usize, l: usize| {
            if i == l - 1 { format!("{}]", val) } else { format!("{}, ", val) }
        };
        let result: String = vec![10, 20, 30].into_par().orx_par_iter_string_fn_clone(f);
        assert_eq!("10, 20, 30]", result);

        let st = "S".to_string();
        let f2 = |s: &String, val: &str, _i: usize, _l: usize| format!("{}{}", s, val);
        let result2: String = vec![10, 20, 30].into_par().orx_par_iter_string_with_state_fn_clone(st, f2);
        assert_eq!("S10S20S30", result2);

        let rule = |val: &str, _i: usize, _l: usize| format!("<{}>", val);
        let result3: String = vec![10, 20, 30].into_par().orx_par_iter_string_rule_ref_clone(rule);
        assert_eq!("<10><20><30>", result3);

        let st2 = "P".to_string();
        let rule2 = |s: &String, val: &str, _i: usize, _l: usize| format!("{}{}", s, val);
        let result4: String = vec![10, 20, 30].into_par().orx_par_iter_string_with_state_rule_ref_clone(st2, rule2);
        assert_eq!("P10P20P30", result4);
    }

    #[cfg(feature = "orx_parallel")]
    #[test]
    fn test_orx_par_nested_clone_traits() {
        use orx_parallel::*;
        let f = |val: &str, _i: usize, _l: usize| format!("{}|", val);
        let result: String = vec![vec![1, 2], vec![3]].into_par().orx_par_iter_string_fn_clone_nested(DEFAULT_FORMAT_RULE, f);
        assert_eq!("[1, 2]|[3]|", result);

        let st = "X".to_string();
        let f2 = |s: &String, val: &str, _i: usize, _l: usize| format!("{}{}", s, val);
        let result2: String = vec![vec![1, 2], vec![3]].into_par().orx_par_iter_string_with_state_fn_clone_nested(DEFAULT_FORMAT_RULE, st, f2);
        assert_eq!("X[1, 2]X[3]", result2);

        let rule = |val: &str, _i: usize, _l: usize| format!("<{}>", val);
        let result3: String = vec![vec![1, 2], vec![3]].into_par().orx_par_iter_string_rule_ref_clone_nested(DEFAULT_FORMAT_RULE, rule);
        assert_eq!("<[1, 2]><[3]>", result3);

        let st2 = "Y".to_string();
        let rule2 = |s: &String, val: &str, _i: usize, _l: usize| format!("{}{}", s, val);
        let result4: String = vec![vec![1, 2], vec![3]].into_par().orx_par_iter_string_with_state_rule_ref_clone_nested(DEFAULT_FORMAT_RULE, st2, rule2);
        assert_eq!("Y[1, 2]Y[3]", result4);
    }

    #[cfg(all(feature = "orx_parallel", feature = "dyn_async"))]
    mod orx_clone_dyn_async_tests {
        use super::*;
        use alloc::boxed::Box;
        use core::future::Future;
        use orx_parallel::*;

        fn block_on_dyn<T>(fut: Box<dyn Future<Output = T>>) -> T {
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

        fn noop_raw_waker() -> core::task::RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            core::task::RawWaker::new(core::ptr::null(), &VTABLE)
        }

        #[test]
        fn test_orx_dyn_async_clone_fn() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_dyn(
                OrxParIteratorStringFnAsyncClone::orx_par_iter_string_async_fn_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_dyn_async_clone_fn_send() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_dyn(
                OrxParIteratorStringFnAsyncCloneSend::orx_par_iter_string_async_fn_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_dyn_async_clone_fn_mut() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_dyn(
                OrxParIteratorStringFnMutAsyncClone::orx_par_iter_string_async_fn_mut_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_dyn_async_clone_fn_mut_send() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_dyn(
                OrxParIteratorStringFnMutAsyncCloneSend::orx_par_iter_string_async_fn_mut_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }
    }

    #[cfg(all(feature = "orx_parallel", feature = "impl_async"))]
    mod orx_clone_impl_async_tests {
        use super::*;
        use core::future::Future;
        use orx_parallel::*;

        fn block_on_impl<F: Future>(mut fut: F) -> F::Output {
            let mut fut = unsafe { core::pin::Pin::new_unchecked(&mut fut) };
            let waker = noop_raw_waker();
            let mut cx = core::task::Context::from_waker(&waker);
            loop {
                if let core::task::Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                    return val;
                }
                core::hint::spin_loop();
            }
        }

        fn noop_raw_waker() -> core::task::Waker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> core::task::RawWaker {
                core::task::RawWaker::new(p, &VTABLE)
            }
            static VTABLE: core::task::RawWakerVTable =
                core::task::RawWakerVTable::new(clone, no_op, no_op, no_op);
            let raw = core::task::RawWaker::new(core::ptr::null(), &VTABLE);
            unsafe { core::task::Waker::from_raw(raw) }
        }

        #[test]
        fn test_orx_impl_async_clone_fn() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_impl(
                OrxParIteratorStringFnImplAsyncClone::orx_par_iter_string_async_fn_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_impl_async_clone_fn_send() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_impl(
                OrxParIteratorStringFnImplAsyncCloneSend::orx_par_iter_string_async_fn_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_impl_async_clone_fn_mut() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_impl(
                OrxParIteratorStringFnMutImplAsyncClone::orx_par_iter_string_async_fn_mut_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }

        #[test]
        fn test_orx_impl_async_clone_fn_mut_send() {
            let f = |val: &str, _i: usize, _l: usize| {
                let val = val.to_string();
                async move { format!("[{}]", val) }
            };
            let result = block_on_impl(
                OrxParIteratorStringFnMutImplAsyncCloneSend::orx_par_iter_string_async_fn_mut_clone(vec![1, 2, 3].into_par(), f),
            );
            assert_eq!("[1][2][3]", result);
        }
    }

    // ========================================================================
    // 11. EDGE CASES
    // ========================================================================
    #[test]
    fn test_nested_empty_inner() {
        let v: Vec<Vec<i32>> = vec![vec![], vec![1]];
        // empty vec formats to "" via vec_string, so nested gives "[, [1]]"
        assert_eq!(
            "[, [1]]",
            v.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_nested_empty_outer() {
        let v: Vec<Vec<i32>> = vec![];
        assert_eq!(
            "",
            v.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_nested_single_element() {
        let v = vec![vec![42]];
        assert_eq!(
            "[[42]]",
            v.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
        );
    }

    #[test]
    fn test_nested_custom_format_rule() {
        let v = vec![vec![1, 2], vec![3]];
        let rule = |val: &str, _i: usize, _l: usize| format!("{}|", val);
        assert_eq!(
            "[1, 2]|[3]|",
            v.vec_string_nested(DEFAULT_FORMAT_RULE, rule)
        );
    }
}
