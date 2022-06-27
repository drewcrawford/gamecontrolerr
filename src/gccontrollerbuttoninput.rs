use objr::bindings::*;
objc_class! {
    pub struct GCControllerButtonInput {
        @class(GCControllerButtonInput)
    }
}

objc_selector_group! {
    trait Selectors {
        @selector("isPressed")
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
}