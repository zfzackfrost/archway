use archway::*;

pub trait TestTrait {
    fn value(&self) -> i32;
}
pub struct First(pub i32, pub i32);
impl TestTrait for First {
    fn value(&self) -> i32 {
        self.0
    }
}
pub struct Second(pub i32, pub i32);
impl TestTrait for Second {
    fn value(&self) -> i32 {
        self.1
    }
}

pub fn eq_shared_value_from<P: SharedPointer<i32>>(value: i32) {
    let shared = P::from(value);
    assert_eq!(42_i32, shared.as_ref().clone())
}
pub fn eq_shared_value<T: Clone + PartialEq + core::fmt::Debug, P: AnySharedPointer<T>>(value: T, shared: P) {
    assert_eq!(value, shared.as_ref().clone())
}
pub fn eq_shared_dyn_value<P: AnySharedPointer<dyn TestTrait>>(value: i32, shared: P) {
    assert_eq!(value, shared.value())
}
pub fn eq_weak_value<T: Clone + PartialEq + core::fmt::Debug, P: AnyWeakPointer<T>>(value: T, weak: P) {
    let shared = weak.upgrade().unwrap();
    assert_eq!(value, shared.as_ref().clone())
}
