use objr::bindings::*;
use crate::GCKeyCode;
use crate::GCControllerButtonInput;
objc_class! {
    pub struct GCKeyboardInput {
        @class(GCKeyboardInput)
    }
}
objc_selector_group! {
    trait GCKeyboardInputSel {
        @selector("buttonForKeyCode:")
    }
    impl GCKeyboardInputSel for Sel {}
}

#[allow(non_snake_case)]
impl GCKeyboardInput {
    pub fn buttonForKeyCode(&self, code: GCKeyCode, pool: &ActiveAutoreleasePool) -> Option<StrongCell<GCControllerButtonInput>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::buttonForKeyCode_(), pool, (code.0,));
            GCControllerButtonInput::nullable(raw).assume_retained()
        }
    }
}
