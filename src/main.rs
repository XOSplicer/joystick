extern crate glfw;

use glfw::{JoystickId};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let joystick = glfw.get_joystick(JoystickId::Joystick1);
    while true {
        println!("Present: {:?}", joystick.is_present());
        println!("Name: {:?}", joystick.get_name());
        println!("Axes: {:?}", joystick.get_axes());
        println!("Buttons: {:?}", joystick.get_buttons());
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
