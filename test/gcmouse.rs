use objr::bindings::autoreleasepool;
use gamecontrollerr::GCMouse;
use gamecontrollerr::GCControllerElement;
use foundationr::{NSRunLoop,NSDate,ObjcClass,NSObjectTrait};
fn main() {
    autoreleasepool(|pool| {
        let mut mouse = None;
        //maybe this takes longer on github's ci?
        for _ in 0..100_000 {
            mouse = GCMouse::current(pool);
            if mouse.is_some() { break }
            let date = NSDate::class().alloc_init(pool);
            NSRunLoop::mainRunLoop(pool).runUntilDate(&date, pool);
        }
        assert!(mouse.is_some(), "Mouse never arrived");
        let input = mouse.unwrap().mouseInput(pool).unwrap();

        println!("left button pressed: {}",input.leftButton(pool).isPressed(pool));
        println!("middle button: {:?}",input.middleButton(pool));
        println!("right button: {:?}",input.rightButton(pool));
        println!("right button name: {:?}",input.rightButton(pool).unwrap().as_element().unmappedLocalizedName(pool));
        for button in input.auxiliaryButtons(pool).iter(pool) {
            println!("auxiliary button: {:?}",button);
            println!("auxiliary button description {:?}",button.description(pool).to_str(pool));
            println!("button name: {:?}",button.as_element().unmappedLocalizedName(pool));
            println!("collection: {:?}",button.as_element().collection(pool))
        }
    });
}