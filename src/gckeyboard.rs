use objr::bindings::*;
objr::objc_class! {
    pub struct GCKeyboard {
        @class(GCKeyboard)
    }
}
objr::objc_selector_group! {
    trait Selectors {
            @selector("coalescedKeyboard")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl GCKeyboard {
    /**
    As an implementation detail, the first couple of times you call this, it returns `nil`.  To fix this, you have to
    1.  Call it
    2.  Pump the main runloop a few times
    3.  Eventually it starts returning non-nil.

    I'm afraid there is no documentation of this anywhere except what you're reading now.  FB9946714
    */
    pub fn coalescedKeyboard(pool: &ActiveAutoreleasePool) -> Option<StrongMutCell<GCKeyboard>> {
        unsafe {
            let raw = Class::<Self>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::coalescedKeyboard(), pool, ());
            println!("raw is {:?}",raw);
            GCKeyboard::nullable(raw).assume_retained().assume_mut()
        }
    }
}
