# vec-string

Composable formatting for `Vec<T>`, slices, iterators, and parallel collections where `T: Display`.

Normally you can do `format!("{:?}", vec)` when elements implement `Debug`.
This crate provides the same capability for `Display`, with full control over formatting rules, nesting, state, async, and parallelism.

```rust
use vec_string::*;

assert_eq!("[1, 2, 3]", vec![1, 2, 3].vec_string(DEFAULT_FORMAT_RULE));
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | ✓ | Standard library support (disable for `no_std`) |
| `rayon` | | Parallel formatting via rayon |
| `orx_parallel` | | Parallel formatting via orx-parallel (sync + async) |
| `dyn_async` | | Async formatting with `Box<dyn Future>` return type |
| `impl_async` | | Async formatting with `impl Future` return type |
| `ambassador_delegatable` | | Marks traits as `#[delegatable_trait]` for ambassador |

## Quick Start

### Basic formatting

```rust
use vec_string::*;

let v = vec![1, 2, 3];

// Default rule: "[1, 2, 3]"
assert_eq!("[1, 2, 3]", v.vec_string(DEFAULT_FORMAT_RULE));

// Custom closure: (value, index, length) -> String
let csv = |val: &str, _i: usize, _l: usize| format!("{},", val);
assert_eq!("1,2,3,", v.vec_string_fn(csv));

// Mutable closure (stateful formatting)
let mut counter = 0;
let numbered = |val: &str, _i: usize, _l: usize| {
    counter += 1;
    format!("{}:{}", counter, val)
};
assert_eq!("1:1 2:2 3:3", v.vec_string_fn_mut(numbered));
```

### With state

```rust
use vec_string::*;

let v = vec![10, 20, 30];
let prefix = "val".to_string();

let rule = |s: &String, val: &str, _i: usize, _l: usize| format!("{}={}", s, val);
assert_eq!("val=10 val=20 val=30", v.vec_string_with_state_fn(&prefix, rule));
```

### Nested vectors

```rust
use vec_string::*;

let v = vec![vec![1, 2], vec![3]];

// Inner and outer rules
assert_eq!(
    "[[1, 2], [3]]",
    v.vec_string_nested(DEFAULT_FORMAT_RULE, DEFAULT_FORMAT_RULE)
);
```

### Iterators

```rust
use vec_string::*;

// Any iterator with Display items
let result: String = (1..=3).map(|x| x * 10).iterator_string(DEFAULT_FORMAT_RULE);
assert_eq!("[10, 20, 30]", result);

// Exact-size iterators (no intermediate allocation)
let result: String = [1, 2, 3].into_iter().iterator_string_exact(DEFAULT_FORMAT_RULE);
assert_eq!("[1, 2, 3]", result);
```

### Rayon parallel

```rust
use vec_string::*;
use rayon::prelude::*;

let v = vec![1, 2, 3];
let result: String = v.into_par_iter().par_iter_string_fn(|val: &str, _i: usize, _l: usize| {
    format!("[{}]", val)
});
assert_eq!("[1][2][3]", result);
```

### orx-parallel

```rust
use vec_string::*;
use orx_parallel::*;

let v = vec![1, 2, 3];

// Sync
let result: String = v.into_par().orx_par_iter_string_fn(|val: &str, _i: usize, _l: usize| {
    format!("<{}>", val)
});
assert_eq!("<1><2><3>", result);
```

### orx-parallel + async

```rust
use vec_string::*;
use orx_parallel::*;

let v = vec![1, 2, 3];

// dyn async (Box<dyn Future>)
let fut = v.into_par().orx_par_iter_string_async_fn(|val: &str, _i: usize, _l: usize| {
    let val = val.to_string();
    async move { format!("[{}]", val) }
});
// let result = fut.await;

// impl async (impl Future — zero-cost abstraction)
let fut = v.into_par().orx_par_iter_string_async_fn(|val: &str, _i: usize, _l: usize| {
    let val = val.to_string();
    async move { format!("[{}]", val) }
});
// let result = fut.await;
```

### Clone variants (no Sync required)

For cases where the closure cannot implement `Sync`, use the `*_clone` variants.
The closure is taken by value and cloned per iteration:

```rust
use vec_string::*;
use orx_parallel::*;

let v = vec![1, 2, 3];

// F: Clone instead of F: Sync
let result: String = v.into_par().orx_par_iter_string_fn_clone(|val: &str, _i: usize, _l: usize| {
    format!("{}!", val)
});
assert_eq!("1!2!3!", result);
```

### Async formatting (sequential)

```rust
use vec_string::*;

let v = vec![1, 2, 3];

// dyn async
let fut = v.vec_string_fn_async(|val: &str, _i: usize, _l: usize| {
    let val = val.to_string();
    Box::new(async move { format!("[{}]", val) }) as Box<dyn core::future::Future<Output = String>>
});
// let result = fut.await;
```

## Architecture

### Format Rules

The formatting logic is abstracted via rule traits:

| Trait | Signature | Use case |
|-------|-----------|----------|
| `FormatRuleNoState` | `&self, &str, usize, usize -> String` | Immutable closure / fn pointer |
| `FormatRuleMutNoState` | `&mut self, &str, usize, usize -> String` | Mutable closure |
| `FormatRuleNoStateOwned` | `self, &str, usize, usize -> String` | Consuming rule |
| `FormatRule<S>` | `&self, &S, &str, usize, usize -> String` | With immutable state |
| `FormatRuleMut<S>` | `&mut self, &mut S, &str, usize, usize -> String` | With mutable state |

All closures with matching signatures get blanket impls automatically.

### Trait Families

Each formatting operation exists in multiple variants:

- **Fn** — `Fn(&str, usize, usize) -> String` (shared reference, `Sync` for parallel)
- **FnMut** — `FnMut(...)` (mutable, sequential only)
- **FnPtr** — `fn(...)` (function pointer, always `Copy + Sync`)
- **RuleRef / RuleOwned** — trait object via `&R` or `R`
- **WithState** — additional state parameter
- **Clone** — `F: Clone` instead of `F: Sync` (orx-parallel only, closure cloned per item)
- **Async** — `dyn_async` (`Box<dyn Future>`) and `impl_async` (`impl Future`)
- **Send** — `+ Send` bounds for multi-threaded executors

### Iterator Coherence (nightly branch)

This branch uses the `NotVec` auto trait (requires nightly `auto_traits` + `negative_impls`):

```rust
pub auto trait NotVec {}
impl<T, A: Allocator> !NotVec for Vec<T, A> {}
```

This allows blanket impls on `I: Iterator + NotVec` without conflicting with `Vec<T>` impls,
covering all iterator adapters automatically without manual listing.

### no_std

Disable default features and enable only what you need:

```toml
[dependencies]
vec-string = { version = "0.2", default-features = false, features = ["dyn_async"] }
```

The crate uses `alloc` (Vec, String, Box) but does not require `std`.

## Branches

| Branch | Rust | Iterator strategy | Extra features |
|--------|------|-------------------|----------------|
| `master` | Stable | `StableIter` marker trait with manual impls | `itertools`, `itermore` |
| `nightly` | Nightly | `NotVec` auto trait (automatic coverage) | — |

## License

```
Copyright (C) 2024 Cody Bloemhard

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
