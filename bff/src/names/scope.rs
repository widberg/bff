use std::cell::{Cell, RefCell};
use std::ptr::NonNull;

use super::{NameContext, NameType};

#[derive(Copy, Clone, PartialEq, Eq)]
enum BorrowState {
    Idle,
    Shared,
    Exclusive,
}

enum Slot {
    Shared(NonNull<NameContext>),
    Exclusive {
        ptr: NonNull<NameContext>,
        borrow: Cell<BorrowState>,
    },
}

thread_local! {
    static STACK: RefCell<Vec<Slot>> = const { RefCell::new(Vec::new()) };
}

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        STACK.with(|s| {
            s.borrow_mut().pop();
        });
    }
}

pub(super) fn scope<R>(ctx: &NameContext, f: impl FnOnce() -> R) -> R {
    STACK.with(|s| {
        s.borrow_mut().push(Slot::Shared(NonNull::from(ctx)));
    });
    let _guard = Guard;
    f()
}

pub(super) fn scope_mut<R>(ctx: &mut NameContext, f: impl FnOnce() -> R) -> R {
    STACK.with(|s| {
        s.borrow_mut().push(Slot::Exclusive {
            ptr: NonNull::from(ctx),
            borrow: Cell::new(BorrowState::Idle),
        });
    });
    let _guard = Guard;
    f()
}

// Inspect the topmost slot, optionally claim a borrow on it, and return what
// we need to dereference the pointer outside the STACK borrow. The slot index
// is returned for Exclusive slots so the caller can clear the flag later via
// a fresh STACK borrow — this avoids holding a reference into the Vec across
// the user closure (during which nested scopes may reallocate the Vec).
fn claim_top_for_shared() -> Option<(NonNull<NameContext>, Option<usize>)> {
    STACK.with(|s| {
        let stack = s.borrow();
        let idx = stack.len().checked_sub(1)?;
        match &stack[idx] {
            Slot::Shared(p) => Some((*p, None)),
            Slot::Exclusive { ptr, borrow } => {
                if borrow.get() == BorrowState::Exclusive {
                    panic!("with_name_context: NameContext is already mutably borrowed");
                }
                borrow.set(BorrowState::Shared);
                Some((*ptr, Some(idx)))
            }
        }
    })
}

fn claim_top_for_exclusive() -> Option<(NonNull<NameContext>, usize)> {
    STACK.with(|s| {
        let stack = s.borrow();
        let idx = stack.len().checked_sub(1)?;
        match &stack[idx] {
            Slot::Shared(_) => None,
            Slot::Exclusive { ptr, borrow } => {
                if borrow.get() != BorrowState::Idle {
                    panic!("with_name_context_mut: NameContext is already borrowed");
                }
                borrow.set(BorrowState::Exclusive);
                Some((*ptr, idx))
            }
        }
    })
}

fn release_borrow(idx: usize) {
    STACK.with(|s| {
        if let Some(Slot::Exclusive { borrow, .. }) = s.borrow().get(idx) {
            borrow.set(BorrowState::Idle);
        }
    });
}

pub(super) fn with_name_context<R>(f: impl FnOnce(Option<&NameContext>) -> R) -> R {
    let Some((ptr, claimed_idx)) = claim_top_for_shared() else {
        return f(None);
    };
    // SAFETY: `ptr` was registered by an enclosing `scope`/`scope_mut` whose
    // Guard has not yet dropped, so the pointee is alive. For Exclusive slots
    // we have just set the borrow flag to Shared, so any nested
    // `with_name_context_mut` on this slot will panic before producing an
    // aliasing `&mut`. For Shared slots the original `&NameContext` is
    // upheld by Rust's borrow checker around the enclosing `scope` call.
    let r = unsafe { ptr.as_ref() };
    let out = f(Some(r));
    if let Some(idx) = claimed_idx {
        release_borrow(idx);
    }
    out
}

pub(super) fn with_name_context_mut<R>(f: impl FnOnce(Option<&mut NameContext>) -> R) -> R {
    let Some((mut ptr, idx)) = claim_top_for_exclusive() else {
        return f(None);
    };
    // SAFETY: `ptr` originated from `scope_mut(&mut self, ...)` whose Guard
    // has not yet dropped, so the pointee is alive and uniquely owned by the
    // active `scope_mut`. We just set the borrow flag to Exclusive, so any
    // nested `with_name_context` or `with_name_context_mut` on this slot will
    // panic before aliasing.
    let r = unsafe { ptr.as_mut() };
    let out = f(Some(r));
    release_borrow(idx);
    out
}

pub(crate) fn current_name_type() -> Option<NameType> {
    with_name_context(|c| c.map(NameContext::name_type))
}

#[cfg(test)]
mod tests {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use super::super::{NameContext, NameType};
    use super::*;

    fn make(name_type: NameType) -> NameContext {
        NameContext::new(name_type)
    }

    fn stack_len() -> usize {
        STACK.with(|s| s.borrow().len())
    }

    #[test]
    fn no_scope_returns_none() {
        assert!(with_name_context(|c| c.is_none()));
        assert!(with_name_context_mut(|c| c.is_none()));
        assert_eq!(current_name_type(), None);
    }

    #[test]
    fn scope_makes_context_visible_then_clears() {
        let ctx = make(NameType::Asobo32);
        ctx.scope(|| {
            assert!(with_name_context(|c| c.is_some()));
            assert_eq!(current_name_type(), Some(NameType::Asobo32));
        });
        assert!(with_name_context(|c| c.is_none()));
        assert_eq!(stack_len(), 0);
    }

    #[test]
    fn scope_mut_offers_both_shared_and_exclusive() {
        let mut ctx = make(NameType::Asobo32);
        ctx.scope_mut(|| {
            assert!(with_name_context(|c| c.is_some()));
            assert!(with_name_context_mut(|c| c.is_some()));
        });
        assert!(with_name_context(|c| c.is_none()));
        assert!(with_name_context_mut(|c| c.is_none()));
    }

    #[test]
    fn scope_returns_none_for_mut() {
        let ctx = make(NameType::Asobo32);
        ctx.scope(|| {
            assert!(with_name_context_mut(|c| c.is_none()));
        });
    }

    #[test]
    fn nested_scopes_lifo_with_distinct_types() {
        let outer = make(NameType::Asobo32);
        let inner = make(NameType::Asobo64);
        outer.scope(|| {
            assert_eq!(current_name_type(), Some(NameType::Asobo32));
            inner.scope(|| {
                assert_eq!(current_name_type(), Some(NameType::Asobo64));
            });
            assert_eq!(current_name_type(), Some(NameType::Asobo32));
        });
        assert_eq!(current_name_type(), None);
    }

    #[test]
    fn nested_scope_mut_inside_scope_works() {
        let outer = make(NameType::Asobo32);
        let mut inner = make(NameType::Asobo64);
        outer.scope(|| {
            inner.scope_mut(|| {
                assert!(with_name_context_mut(|c| c.is_some()));
                assert_eq!(current_name_type(), Some(NameType::Asobo64));
            });
            // Outer is shared, mut not available.
            assert!(with_name_context_mut(|c| c.is_none()));
            assert_eq!(current_name_type(), Some(NameType::Asobo32));
        });
    }

    #[test]
    fn panic_in_scope_unwinds_and_pops_stack() {
        let ctx = make(NameType::Asobo32);
        let result = catch_unwind(AssertUnwindSafe(|| {
            ctx.scope(|| {
                panic!("boom");
            });
        }));
        assert!(result.is_err());
        assert_eq!(stack_len(), 0);
        assert!(with_name_context(|c| c.is_none()));
    }

    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn shared_inside_mut_panics() {
        let mut ctx = make(NameType::Asobo32);
        ctx.scope_mut(|| {
            with_name_context_mut(|outer| {
                let _outer = outer.expect("mut should be available");
                with_name_context(|_| {});
            });
        });
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn mut_inside_shared_panics() {
        let mut ctx = make(NameType::Asobo32);
        ctx.scope_mut(|| {
            with_name_context(|outer| {
                let _outer = outer.expect("shared should be available");
                with_name_context_mut(|_| {});
            });
        });
    }

    #[test]
    fn distinct_slots_do_not_conflict() {
        let a = make(NameType::Asobo32);
        let mut b = make(NameType::Asobo64);
        a.scope(|| {
            b.scope_mut(|| {
                with_name_context_mut(|m| {
                    let m = m.expect("top is exclusive");
                    assert_eq!(m.name_type(), NameType::Asobo64);
                });
                with_name_context(|s| {
                    let s = s.expect("top still resolvable");
                    assert_eq!(s.name_type(), NameType::Asobo64);
                });
            });
        });
    }

    #[test]
    fn current_name_type_tracks_topmost() {
        let outer = make(NameType::Asobo32);
        let inner = outer.into_retyped(NameType::Asobo64);
        let outer = make(NameType::Asobo32);
        outer.scope(|| {
            assert_eq!(current_name_type(), Some(NameType::Asobo32));
            inner.scope(|| {
                assert_eq!(current_name_type(), Some(NameType::Asobo64));
            });
            assert_eq!(current_name_type(), Some(NameType::Asobo32));
        });
    }

    #[test]
    fn insert_inside_scope_mut_is_resolvable() {
        let mut ctx = make(NameType::Asobo32);
        let inserted = ctx.scope_mut(|| {
            with_name_context_mut(|c| {
                let c = c.expect("mut available");
                c.insert("hello_inserted_name")
            })
        });
        assert_eq!(
            ctx.resolve(inserted).as_deref(),
            Some("hello_inserted_name")
        );
    }
}
