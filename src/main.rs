extern crate glfw;

use glfw::{Joystick, JoystickId};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not load GLFW!");
    let joystick = JoystickHandle(glfw.get_joystick(JoystickId::Joystick1));
    while true {
        println!("Present: {:?}", joystick.get_data());
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

struct JoystickHandle(Joystick);

#[derive(Debug)]
struct JoystickData {
    name: String,
    axes: Vec<f32>,
    buttons: Vec<bool>,
}

impl JoystickHandle {
    fn get_data(&self) -> Result<JoystickData, String> {
        match self.0.is_present() {
            true => Ok(JoystickData {
                    name: self.0.get_name(),
                    axes: self.0.get_axes(),
                    buttons: self.0.get_buttons().iter().map(|&x| x!=0).collect()
            }),
            false => Err("Joystick not present".to_owned())
        }
    }
}
