# A non-`'static` `downcast` implementation

Given a type `T`, this type outlives some livetime `'T`.
Each type `T` may have zero or more lifetime parameters, `'0`, `'1`, ...
Each lifetime parameter MUST (be definition) `'n: 'T` (where `'n` is `'0` etc)
  In fact, `'T` is constructed by taking the largest lifetime that meets the above criteria

In general, it is safe to cast `'a` to `'b` if `'a: 'b`.

Therefore, it is safe to cast an object of type `T` to `T<'a, 'a, ...>`, where `'a` is shorter than
any of the lifetime parameters (and `'T`).
  Ignoring type aliases, this can be done by simply forcing each parameter to be `'a`.

The following auto-trait could facilitate this:
```rust
unsafe trait NonStaticType<'a> {
  /// A version of this type where all lifetime references have been cast to `'a`
  type LoweredType;
}
unsafe impl<'a> NonStaticType<'a> for &str {
  type LoweredType = &'a str;
}
```

Assuming this auto-trait is implemented properly, the following code would be safe:
```rust
fn downcast_non_static<'a, T: NonStaticType + 'a>(val: &'a T) -> &'a T::LoweredType {
  unsafe { std::mem::transmute(val) }
}
```
