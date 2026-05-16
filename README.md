# funcorder-rs

A linter based on the Go Uber style guideline [Function Grouping And Ordering][uber-function].

## What it does

Checks that methods within an `impl` block are ordered consistently:
constructors (`pub fn new() -> Self`) first, then public methods,
and then private methods.

## Why is this bad?

Following a consistent order for methods within an `impl` block improves readability
and maintainability. Constructors are often the entry point for creating instances
of a struct, so placing them first makes the API clearer. A logical grouping
of methods by visibility further enhances code comprehension.

<!--
### Known problems

Remove if none.
-->

## Example

```rust
struct MyStruct;

impl MyStruct {
    fn private_method(&self) {} // Bad: Private method before constructor

    pub fn constructor() -> Self {
        MyStruct
    }

    pub(crate) fn crate_method(&self) {} // Bad: Crate method before public

    pub fn public_method(&self) {}
}
```

Use instead:

```rust
struct MyStruct;

impl MyStruct {
    pub fn constructor() -> Self {
        MyStruct
    }

    pub fn public_method(&self) {}

    pub(crate) fn crate_method(&self) {}

    fn private_method(&self) {}
}
```

## How to use it

**funcorder-rs** is using [dylint][dylint]. Check in their GitHub page how to use it.

For reference:

```toml
[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/manuelarte/funcorder-rs" },
]
```


[dylint]: https://github.com/trailofbits/dylint/
[uber-function]: https://github.com/uber-go/guide/blob/master/style.md#function-grouping-and-ordering
