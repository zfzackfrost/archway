extern crate alloc;
use alloc::sync::Arc;

mod common;
use common::*;

use archway::*;

#[test]
fn test_shared_value() {
    let shared = Arc::new(42_i32);
    eq_shared_value(42_i32, shared);
}

#[test]
fn test_shared_value_from() {
    eq_shared_value_from::<Arc<_>>(42_i32);
}
#[test]
fn test_shared_value_from_kind() {
    eq_shared_value_from_kind::<ArcKind>(42_i32);
}
#[test]
fn test_weak_value() {
    let shared = Arc::new(42_i32);
    eq_weak_value(42_i32, shared.downgrade());
}
