extern crate glfw;

mod joystick;
use joystick::*;

use std::time::Duration;
use std::sync::mpsc;
use glfw::JoystickId;

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not load GLFW!");
    let mut builder = JoystickThreadBuilder::new(glfw,
                                                 JoystickId::Joystick1,
                                                 Duration::from_millis(5));
    let (tx, rx) = mpsc::channel::<()>();
    for button in 0_usize..8 {
        builder.on_press(button, Some(Box::new(move |_| println!("Button {} pressed", button))))
                .on_release(button, Some(Box::new(move |_| println!("Button {} released", button))));
    }
    for axis in 0_usize..4 {
        builder.on_move(axis, Some(Box::new(move |v| println!("Axis {} moved to {}", axis, v))));
    }
    builder.on_release(9, Some(Box::new(move |_| {
                                                println!("Button 9 released");
                                                tx.send(()).unwrap();
                                            })));
    let thread = builder.spin_up();
    rx.recv().unwrap(); //block until button 9 was pressed 
    thread.tear_down().unwrap();
}
