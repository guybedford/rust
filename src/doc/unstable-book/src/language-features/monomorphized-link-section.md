# `monomorphized_link_section`

The tracking issue for this feature is: [#158620](https://github.com/rust-lang/rust/issues/158620).

------------------------

Normally `#[link_section]` and `#[used]` may only be applied to `static` items,
and `const` items are always inlined at their use sites without receiving an
address of their own.

The `monomorphized_link_section` feature allows `#[link_section]` (and `#[used]`)
to be applied to a *generic* `const` item. Each monomorphization that is actually
instantiated is then *reified* into its own read-only static:

* one symbol is emitted per distinct instantiation, named by the normal
  monomorphized mangling of the const,
* the static's initializer is the const-evaluated bytes for that instantiation,
* the static is placed in the requested `#[link_section]`,
* identical instantiations are emitted with `linkonce_odr` linkage so the linker
  merges them, while distinct instantiations get distinct symbols and are
  concatenated in the section,
* `#[used]` retains every instantiation even though the const is only referenced
  at const-evaluation time.

This lets a post-link tool read a whole named section as a concatenation of
per-monomorphization blobs.

The type-dependent length case (`[u8; N::<T>()]`) additionally requires
[`generic_const_exprs`]; with a fixed size only [`generic_const_items`] is
needed.

[`generic_const_items`]: ./generic-const-items.md
[`generic_const_exprs`]: ./generic-const-exprs.md

```rust
#![feature(generic_const_items, monomorphized_link_section)]
#![allow(incomplete_features)]

#[used]
#[link_section = "__descriptors"]
const DESC<T>: [u8; 4] = [1, 2, 3, 4];

fn read<T>() -> u8 {
    DESC::<T>[0]
}

fn main() {
    let _ = read::<u32>();
    let _ = read::<u64>();
}
```
