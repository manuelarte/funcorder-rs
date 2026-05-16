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
    fn do_something_private(&self) {} // Bad: Private method before constructor

    pub fn new() -> Self {
        MyStruct
    }

    pub(crate) fn do_something_crate_private(&self) {} // Bad: Crate method before public

    pub fn do_something_public(&self) {}
}
```

Use instead:

```rust
struct MyStruct;

impl MyStruct {
    pub fn new() -> Self {
        MyStruct
    }

    pub fn do_something_public(&self) {}

    pub(crate) fn do_something_crate_private(&self) {}

    fn do_something_private(&self) {}
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
