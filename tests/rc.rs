extern crate alloc;
use alloc::rc::Rc;

mod common;
use common::*;

use archway::*;

#[test]
fn test_shared_value() {
    let shared = Rc::new(42_i32);
    eq_shared_value(42_i32, shared);
}
#[test]
fn test_shared_dyn_value() {
    let shared: Rc<dyn TestTrait> = Rc::new(First(10, 20));
    eq_shared_dyn_value(10, shared);

    let shared: Rc<dyn TestTrait> = Rc::new(Second(10, 20));
    eq_shared_dyn_value(20, shared);
}
#[test]
fn test_shared_value_from() {
    eq_shared_value_from::<Rc<_>>(42_i32);
}
#[test]
fn test_shared_value_from_kind() {
    eq_shared_value_from_kind::<RcKind>(42_i32);
}
#[test]
fn test_weak_value() {
    let shared = Rc::new(42_i32);
    eq_weak_value(42_i32, shared.downgrade());
}
