extern crate glfw;
use glfw::JoystickId;

extern crate clap;
use clap::{Arg, App, SubCommand};

mod joystick;
use joystick::*;

use std::time::Duration;
use std::sync::mpsc;


fn main() {

    let matches = App::new("gamepad_test")
                          .version("0.1")
                          .author("Felix Stegmaier <stegmaier.felix@gmail.com>")
                          .about("Test your gampad with rust")
                          .args_from_usage(
                              "<BUTTONS>      'The amount of Buttons your gamepad has'")
                          .get_matches();

    let buttons = matches.value_of("BUTTONS").unwrap()
                            .parse::<u32>()
                            .expect("BUTTONS must be a positive integer");

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not load GLFW!");
    let mut builder = JoystickThreadBuilder::new(glfw,
                                                 JoystickId::Joystick1,
                                                 Duration::from_millis(5));
    let (tx, rx) = mpsc::channel::<()>();
    for button in 0..buttons {
        builder.on_press(button as usize,
                        Some(Box::new(move |_| println!("Button {} pressed", button))))
                .on_release(button as usize,
                        Some(Box::new(move |_| println!("Button {} released", button))));
    }
    for axis in 0_usize..4 {
        builder.on_move(axis, Some(Box::new(move |v| println!("Axis {} moved to {}", axis, v))));
    }
    builder.on_release((buttons-1) as usize, Some(Box::new(move |_| {
                                                tx.send(()).unwrap();
                                            })));
    let thread = builder.spin_up();
    rx.recv().unwrap(); //block until button 9 was pressed
    thread.tear_down().unwrap();

}
