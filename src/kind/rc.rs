use alloc::rc::{Rc, Weak};

use crate::shared::{AnySharedPointer, SharedPointer};
use crate::weak::AnyWeakPointer;

impl<T: ?Sized> AnySharedPointer<T> for Rc<T> {
    type Weak = Weak<T>;
    fn downgrade(&self) -> Self::Weak {
        Rc::downgrade(self)
    }
}
impl<T: Sized> SharedPointer<T> for Rc<T> {}
impl<T: ?Sized> AnyWeakPointer<T> for Weak<T> {
    type Shared = Rc<T>;
    fn upgrade(&self) -> Option<Self::Shared> {
        Weak::upgrade(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait TestTrait {
        fn value(&self) -> i32;
    }
    pub struct First(i32, i32);
    impl TestTrait for First {
        fn value(&self) -> i32 {
            self.0
        }
    }
    pub struct Second(i32, i32);
    impl TestTrait for Second {
        fn value(&self) -> i32 {
            self.1
        }
    }

    fn eq_shared_value_from<P: SharedPointer<i32>>(value: i32) {
        let shared = P::from(value);
        assert_eq!(42i32, shared.as_ref().clone())
    }
    fn eq_shared_value<T: Clone + PartialEq + core::fmt::Debug, P: AnySharedPointer<T>>(value: T, shared: P) {
        assert_eq!(value, shared.as_ref().clone())
    }
    fn eq_shared_dyn_value<P: AnySharedPointer<dyn TestTrait>>(value: i32, shared: P) {
        assert_eq!(value, shared.value())
    }
    fn eq_weak_value<T: Clone + PartialEq + core::fmt::Debug, P: AnyWeakPointer<T>>(value: T, weak: P) {
        let shared = weak.upgrade().unwrap();
        assert_eq!(value, shared.as_ref().clone())
    }

    #[test]
    fn test_shared_value() {
        use alloc::rc::Rc;
        let shared = Rc::new(42i32);
        eq_shared_value(42i32, shared);
    }
    #[test]
    fn test_shared_dyn_value() {
        use alloc::rc::Rc;

        let shared: Rc<dyn TestTrait> = Rc::new(First(10, 20));
        eq_shared_dyn_value(10, shared);

        let shared: Rc<dyn TestTrait> = Rc::new(Second(10, 20));
        eq_shared_dyn_value(20, shared);
    }
    #[test]
    fn test_shared_value_from() {
        use alloc::rc::Rc;
        eq_shared_value_from::<Rc<_>>(42i32);
    }
    #[test]
    fn test_weak_value() {
        use alloc::rc::Rc;
        let shared = Rc::new(42i32);
        eq_weak_value(42i32, shared.downgrade());
    }
}
