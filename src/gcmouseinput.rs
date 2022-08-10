use objr::bindings::*;
use crate::GCControllerButtonInput;
use foundationr::NSArray;
type GCDeviceButtonInput = GCControllerButtonInput;

objc_class! {
    pub struct GCMouseInput {
        @class(GCMouseInput)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("leftButton")
        @selector("middleButton")
        @selector("rightButton")
        @selector("auxiliaryButtons")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl GCMouseInput {
    pub fn leftButton(&self, pool: &ActiveAutoreleasePool) -> StrongCell<GCDeviceButtonInput> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::leftButton(), pool, ());
            GCDeviceButtonInput::assume_nonnil(raw).assume_retained()
        }
    }
    pub fn middleButton(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<GCDeviceButtonInput>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::middleButton(), pool, ());
            GCDeviceButtonInput::nullable(raw).assume_retained()
        }
    }
    pub fn rightButton(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<GCDeviceButtonInput>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::rightButton(), pool, ());
            GCDeviceButtonInput::nullable(raw).assume_retained()
        }
    }
    pub fn auxiliaryButtons(&self, pool: &ActiveAutoreleasePool) -> StrongCell<NSArray<GCDeviceButtonInput>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::auxiliaryButtons(), pool, ());
            NSArray::assume_nonnil(raw).assume_retained()
        }
    }
}
