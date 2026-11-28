#![feature(rustc_private)]
#![warn(unused_extern_crates)]

mod models;

// A list of available compiler crates can be found here:
// https://doc.rust-lang.org/nightly/nightly-rustc/
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use crate::models::MethodMetadata;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{ImplItemKind, Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass};

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks non-macro-expanded methods in inherent `impl` blocks are ordered
    /// consistently: constructor-like associated functions first, then public
    /// receiver methods, and then restricted-visibility receiver methods.
    ///
    /// ### Why is this bad?
    /// Following a consistent order for methods within an `impl` block improves readability
    /// and maintainability. Constructors are often the entry point for creating instances
    /// of a struct, so placing them first makes the API clearer. A logical grouping
    /// of methods by visibility further enhances code comprehension.
    ///
    /// ### Example
    /// ```no_run
    /// struct MyStruct;
    ///
    /// impl MyStruct {
    ///     fn private_method(&self) {} // Bad: restricted method before constructor
    ///
    ///     pub fn constructor() -> Self {
    ///         MyStruct
    ///     }
    ///
    ///     pub(crate) fn crate_method(&self) {}
    ///
    ///     pub fn public_method(&self) {}
    /// }
    /// ```
    /// Use instead:
    /// ```no_run
    /// struct MyStruct;
    ///
    /// impl MyStruct {
    ///     pub fn constructor() -> Self {
    ///         MyStruct
    ///     }
    ///
    ///     pub fn public_method(&self) {}
    ///
    ///     pub(crate) fn crate_method(&self) {}
    ///
    ///     fn private_method(&self) {}
    /// }
    /// ```
    pub FUNCORDER_RS,
    Warn,
    "Linter that orders constructor-like functions, public methods, and restricted-visibility methods"
}

impl<'tcx> LateLintPass<'tcx> for FuncorderRs {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if let ItemKind::Impl(impl_block) = item.kind {
            if item.span.from_expansion() {
                return;
            }

            // Only check inherent impl blocks (not trait implementations)
            if impl_block.of_trait.is_some() {
                return;
            }

            // Get the `Self` type for the `impl` block
            let self_ty = cx
                .tcx
                .type_of(item.owner_id)
                .instantiate_identity()
                .skip_norm_wip();

            let mut last_category: Option<MethodMetadata> = None;

            for &impl_item_id in impl_block.items {
                let impl_item = cx.tcx.hir_impl_item(impl_item_id);
                if impl_item.span.from_expansion() {
                    continue;
                }

                // Only consider functions for ordering
                if let ImplItemKind::Fn(ref sig, _) = impl_item.kind {
                    let name = impl_item.ident.name;

                    let visibility = cx.tcx.visibility(impl_item.owner_id.def_id);

                    let Some(current_category) = MethodMetadata::from(
                        sig,
                        cx,
                        impl_item.owner_id,
                        self_ty,
                        name,
                        visibility,
                    ) else {
                        continue;
                    };

                    if let Some(lc) = last_category
                        && current_category.priority() < lc.priority()
                    {
                        let msg = format!(
                            "methods are not ordered correctly: `{current_category}` declared after `{lc}`"
                        );
                        span_lint_and_help(
                            cx,
                            FUNCORDER_RS,
                            impl_item.span,
                            msg,
                            None,
                            "reorder functions to be constructors, public, restricted",
                        );
                    }
                    last_category = Some(current_category);
                }
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
