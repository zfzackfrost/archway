use archway::*;

pub fn eq_shared_value_from_kind<P: PointerKind>(value: i32) {
    let shared = P::Shared::<i32>::from(value);
    assert_eq!(42_i32, shared.as_ref().clone())
}
pub fn eq_shared_value_from<P: AnySharedPointer<i32>>(value: i32) {
    let shared = P::from(value);
    assert_eq!(42_i32, shared.as_ref().clone())
}
pub fn eq_shared_value<T: Clone + PartialEq + core::fmt::Debug, P: AnySharedPointer<T>>(value: T, shared: P) {
    assert_eq!(value, shared.as_ref().clone())
}
pub fn eq_weak_value<T: Clone + PartialEq + core::fmt::Debug, P: AnyWeakPointer<T>>(value: T, weak: P) {
    let shared = weak.upgrade().unwrap();
    assert_eq!(value, shared.as_ref().clone())
}
