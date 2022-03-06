use objr::bindings::autoreleasepool;
use gamecontrollerr::GCKeyboard;
use foundationr::{NSRunLoop,NSDate,ObjcClass};
fn main() {
    autoreleasepool(|pool| {
        let mut keyboard = None;
        for i in 0..1000 {
            keyboard = GCKeyboard::coalescedKeyboard(pool);
            if keyboard.is_some() { break }
            let date = foundationr::NSDate::class().alloc_init(pool);
            NSRunLoop::mainRunLoop(pool).runUntilDate(&date, pool);
        }
        assert!(keyboard.is_some(), "Keyboard never arrived")

    });
}