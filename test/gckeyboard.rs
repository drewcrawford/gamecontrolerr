use objr::bindings::autoreleasepool;
use gamecontrollerr::{GCKeyboard, GCKeyCode};
use foundationr::{NSRunLoop,NSDate,ObjcClass};
fn main() {
    autoreleasepool(|pool| {
        let mut keyboard = None;
        for _ in 0..1000 {
            keyboard = GCKeyboard::coalescedKeyboard(pool);
            if keyboard.is_some() { break }
            let date = NSDate::class().alloc_init(pool);
            NSRunLoop::mainRunLoop(pool).runUntilDate(&date, pool);
        }
        assert!(keyboard.is_some(), "Keyboard never arrived");
        let input = keyboard.unwrap().keyboardInput(pool);
        assert!(input.is_some(), "Keyboard had no input");
        let key = GCKeyCode::return_or_enter();
        let button = input.unwrap().buttonForKeyCode(key,pool);
        assert!(button.is_some(), "Can't find 'return' button");
        let button = button.unwrap();
        assert!(!button.isPressed(pool));
    });
}