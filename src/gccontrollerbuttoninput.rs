use blocksr::many_escaping_nonreentrant;
use objr::bindings::*;
use std::ffi::c_float;
use crate::GCControllerElement;
objc_class! {
    pub struct GCControllerButtonInput {
        @class(GCControllerButtonInput)
    }
}


objc_selector_group! {
    trait Selectors {
        @selector("isPressed")
        @selector("setPressedChangedHandler:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl GCControllerButtonInput {
    pub fn isPressed(&self, pool: &ActiveAutoreleasePool) -> bool {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::isPressed(), pool, ())
        }
    }
    pub fn setPressedChangedHandler<E: Arguable,H: FnMut(&mut E, &GCControllerButtonInput, c_float, u8) + Send + 'static>(&self, initial_environment: E, pool: &ActiveAutoreleasePool, handler: H) {
        many_escaping_nonreentrant!(GCControllerButtonValueChangedHandler (environment: &mut E, button: &GCControllerButtonInput, value: c_float, pressed: u8) -> ());
        unsafe impl Arguable for &GCControllerButtonValueChangedHandler {}
        //safe because E is Arguable
        let block = unsafe{GCControllerButtonValueChangedHandler::new(initial_environment, handler)};
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::setPressedChangedHandler_(), pool, (&block,))
        }
    }
}