use clippy_utils::res::MaybeDef;
use clippy_utils::{return_ty, sym};
use rustc_hir::{FnSig, OwnerId};
use rustc_lint::LateContext;
use rustc_middle::ty::{Adt, Ty, Visibility};
use rustc_span::{Symbol, def_id::DefId};
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(super) enum MethodKind {
    Constructor,
    Public,
    Restricted,
}

pub(super) struct MethodMetadata {
    kind: MethodKind,
    name: Symbol,
}

impl MethodMetadata {
    pub(super) fn from<'tcx>(
        fn_sig: &FnSig<'_>,
        cx: &LateContext<'tcx>,
        impl_item_owner_id: OwnerId,
        self_ty: Ty<'tcx>,
        name: Symbol,
        visibility: Visibility<DefId>,
    ) -> Option<Self> {
        if is_constructor(fn_sig, cx, impl_item_owner_id, self_ty) {
            Some(Self {
                kind: MethodKind::Constructor,
                name,
            })
        } else if input_has_self(fn_sig) && visibility.is_public() {
            Some(Self {
                kind: MethodKind::Public,
                name,
            })
        } else if input_has_self(fn_sig) && !visibility.is_public() {
            Some(Self {
                kind: MethodKind::Restricted,
                name,
            })
        } else {
            None
        }
    }

    pub(super) fn priority(&self) -> u8 {
        match self.kind {
            MethodKind::Constructor => 0,
            MethodKind::Public => 1,
            MethodKind::Restricted => 2,
        }
    }
}

impl std::fmt::Display for MethodMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}({})", self.kind, self.name)
    }
}

/// Checks whether an associated function is constructor-like.
///
/// It must have no receiver and return the implementing type directly, as
/// `Option<implementing type>`, or as the first type parameter of `Result`.
fn is_constructor<'tcx>(
    fn_sig: &FnSig<'_>,
    cx: &LateContext<'tcx>,
    impl_item_owner_id: OwnerId,
    self_ty: Ty<'tcx>,
) -> bool {
    if input_has_self(fn_sig) {
        return false;
    }

    // Check if the return type matches the struct type
    let ret_ty = return_ty(cx, impl_item_owner_id);

    // Direct match: returns Self or the struct type
    if ret_ty == self_ty {
        return true;
    }

    // Check for Option<Self> or Result<Self, _>.
    if (ret_ty.is_diag_item(cx, sym::Option) || ret_ty.is_diag_item(cx, sym::Result))
        && let Adt(_, args) = ret_ty.kind()
        && let Some(inner_ty) = args.first().and_then(|arg| arg.as_type())
        && inner_ty == self_ty
    {
        return true;
    }

    false
}

/// Checks whether the function has a receiver (`self`, `&self`, `&mut self`, etc.).
fn input_has_self(fn_sig: &FnSig<'_>) -> bool {
    fn_sig.decl.implicit_self().has_implicit_self()
}
