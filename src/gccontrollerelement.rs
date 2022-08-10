use objr::bindings::*;
use crate::GCControllerButtonInput;
objc_selector_group! {
    trait Selectors {
        @selector("unmappedLocalizedName")
        @selector("collection")
    }
    impl Selectors for Sel {}
}

objc_class! {
    pub struct GCControllerElement {
        @class(GCControllerElement)
    }
}
impl GCControllerElement {
    pub fn unmappedLocalizedName(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSString>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::unmappedLocalizedName(), pool, ());
            NSString::nullable(raw).assume_retained()
        }
    }
    pub fn collection(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<GCControllerElement>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::collection(), pool, ());
            GCControllerElement::nullable(raw).assume_retained()
        }
    }
}
impl<'a> From<&'a GCControllerButtonInput> for &'a GCControllerElement {
    fn from(f: &'a GCControllerButtonInput) -> Self {
        f.as_element()
    }
}
impl GCControllerButtonInput {
    pub fn as_element(&self) -> &GCControllerElement {
        unsafe{std::mem::transmute(self)}
    }
}