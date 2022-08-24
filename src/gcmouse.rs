use objr::bindings::*;
use crate::gcmouseinput::GCMouseInput;
use foundationr::NSArray;
objc_class! {
    pub struct GCMouse {
        @class(GCMouse)
    }
}

objc_selector_group! {
    trait Selectors {
        @selector("current")
        @selector("mouseInput")
        @selector("mice")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl GCMouse {
    pub fn current(pool: &ActiveAutoreleasePool) -> Option<StrongCell<GCMouse>> {
        unsafe {
            let raw = Class::<Self>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::current(), pool, ());
            Self::nullable(raw).assume_retained()
        }
    }
    pub fn mouseInput(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<GCMouseInput>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::mouseInput(), pool, ());
            GCMouseInput::nullable(raw).assume_retained()
        }
    }
    pub fn mice(pool: &ActiveAutoreleasePool) -> StrongCell<NSArray<GCMouse>> {
        unsafe {
            let raw = Class::<Self>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::mice(), pool, ());
            NSArray::assume_nonnil(raw).assume_retained()
        }
    }
}