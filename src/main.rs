extern crate glfw;

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

use glfw::{Joystick, JoystickId, Action};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not load GLFW!");
    let joystick = JoystickHandle(glfw.get_joystick(JoystickId::Joystick1));
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut running = true;
        let mut exit_pressed = false;
        while running {
            let data = joystick.get_data();
            match data {
                Ok(t) => {
                    match t.buttons[9] {
                        Action::Press => exit_pressed = true,
                        Action::Release => {
                            if exit_pressed {
                                running = false;
                            }
                        },
                        _ => panic!("dont hold the button")
                    }
                    tx.send(Message::Data(t)).unwrap();
                },
                Err(e) => {
                    println!("Error: {}", e);
                    running = false;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        tx.send(Message::Exit)
    });
    while let Message::Data(t) = rx.recv().unwrap() {
        println!("{:?}", t);
    }

}

#[derive(Debug, Clone, Copy)]
enum Message<T> {
    Exit,
    Data(T)
}

struct JoystickHandle(Joystick);

#[derive(Debug, Clone)]
struct JoystickData {
    name: String,
    axes: Vec<f32>,
    buttons: Vec<Action>,
}

impl JoystickHandle {
    fn get_data(&self) -> Result<JoystickData, String> {
        match self.0.is_present() {
            true => Ok(JoystickData {
                    name: self.0.get_name(),
                    axes: self.0.get_axes(),
                    buttons: self.0.get_buttons()
                                    .iter().map(|&x| match x {
                                        0 => Action::Release,
                                        1 => Action::Press,
                                        2 => Action::Repeat,
                                        _ => unreachable!()
                                    }).collect()
            }),
            false => Err("Joystick not present".to_owned())
        }
    }
}
