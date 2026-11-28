use std::fmt::Formatter;
use clippy_utils::{return_ty, sym};
use clippy_utils::res::MaybeDef;
use rustc_hir::{FnSig, OwnerId};
use rustc_lint::LateContext;
use rustc_middle::ty::{Adt, Ty, Visibility};
use rustc_span::def_id::DefId;

/// Defines the desired order of method categories.
/// Higher enum variants mean higher priority (should appear earlier).
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum MethodKind {
    Constructor,
    Public,
    Private,
}

pub(crate) struct MethodMetadata {
    kind: MethodKind,
    name: String,
}

impl MethodMetadata {
    pub(crate) fn from<'tcx>(
        fn_sig: &FnSig<'_>,
        cx: &LateContext<'tcx>,
        impl_item_owner_id: OwnerId,
        self_ty: Ty<'tcx>,
        fn_name: &str,
        visibility: Visibility<DefId>,
    ) -> Option<Self> {
        if is_constructor(fn_sig, cx, impl_item_owner_id, self_ty) {
            Some(Self {
                kind: MethodKind::Constructor,
                name: fn_name.to_string(),
            })
        } else if input_has_self(fn_sig) && visibility.is_public() {
            Some(Self {
                kind: MethodKind::Public,
                name: fn_name.to_string(),
            })
        } else if input_has_self(fn_sig) && !visibility.is_public() {
            Some(Self {
                kind: MethodKind::Private,
                name: fn_name.to_string(),
            })
        } else {
            None
        }
    }

    pub(crate) fn kind(&self) -> &MethodKind {
        &self.kind
    }
}

impl std::fmt::Display for MethodMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}({})", self.kind, self.name)
    }
}

/// `is_constructor` check whether the given function is a constructor.
/// A constructor must:
/// - Not have an implicit `self` parameter (&self or &mut self)
/// - Return Self, the struct type, or Option<Self>/Option<struct type>
fn is_constructor<'tcx>(
    fn_sig: &FnSig<'_>,
    cx: &LateContext<'tcx>,
    impl_item_owner_id: OwnerId,
    self_ty: Ty<'tcx>,
) -> bool {
    // Check that the function doesn't take &self or &mut self
    if input_has_self(fn_sig) {
        return false;
    }

    // Check if the return type matches the struct type
    let ret_ty = return_ty(cx, impl_item_owner_id);

    // Direct match: returns Self or the struct type
    if ret_ty == self_ty {
        return true;
    }

    // Check for Option<Self> or Option<struct type>
    if (ret_ty.is_diag_item(cx, sym::Option) || ret_ty.is_diag_item(cx, sym::Result))
        && let Adt(_, args) = ret_ty.kind()
        && let Some(inner_ty) = args.first().and_then(|arg| arg.as_type())
        && inner_ty == self_ty
    {
        return true;
    }

    false
}

/// `input_has_self` checks whether the input parameters of a function contain the `self`
/// parameter in any form.
fn input_has_self(fn_sig: &FnSig<'_>) -> bool {
    fn_sig.decl.implicit_self().has_implicit_self()
}
