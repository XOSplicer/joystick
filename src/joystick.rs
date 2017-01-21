extern crate glfw;

use std::any::Any;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::collections::hash_map::HashMap;

use glfw::{Glfw, Joystick, JoystickId, Action};

type Callback<T> = Option<Box<(Fn(T)) + Send>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoystickError {
    NotPresent
}

#[derive(Debug, Clone, PartialEq)]
pub struct JoystickData {
    name: String,
    axes: Vec<f32>,
    buttons: Vec<Action>,
}

pub struct JoystickThreadBuilder {
    handle: JoystickWrapper,
    on_press: HashMap<usize, Callback<()>>,
    on_release: HashMap<usize, Callback<()>>,
    on_hold: HashMap<usize, Callback<()>>,
    on_move: HashMap<usize, Callback<f32>>,
    polling: Duration,
}

pub struct JoystickThread {
    thread: JoinHandle<()>,
    running: Arc<Mutex<bool>>,
}

struct JoystickWrapper(Joystick);

impl JoystickData {

}

impl JoystickThreadBuilder {

    pub fn new(glfw: Glfw, id: JoystickId, rate: Duration) -> Self {
        JoystickThreadBuilder  {
            handle: JoystickWrapper(glfw.get_joystick(id)),
            on_press: HashMap::new(),
            on_release: HashMap::new(),
            on_hold: HashMap::new(),
            on_move: HashMap::new(),
            polling: rate,
        }
    }

    pub fn on_press(&mut self, button: usize, callback: Callback<()>) -> &mut Self {
        self.on_press.insert(button, callback);
        self
    }

    pub fn on_release(&mut self, button: usize, callback: Callback<()>) -> &mut Self {
        self.on_release.insert(button, callback);
        self
    }

    pub fn on_hold(&mut self, button: usize, callback: Callback<()>) -> &mut Self {
        self.on_hold.insert(button, callback);
        self
    }

    pub fn on_move(&mut self, axis: usize, callback: Callback<f32>) -> &mut Self {
        self.on_move.insert(axis, callback);
        self
    }

    pub fn spin_up(self) -> JoystickThread {
        let running = Arc::new(Mutex::new(true));
        let running_clone = running.clone();
        let joinhandle = thread::spawn(move || {
            let builder = self;
            let mut prev_data = builder.handle.get_data().ok();
            // run until torn down
            while *running.lock().unwrap() {
                //println!("thread is running");
                let data = builder.handle.get_data().ok();
                if data.is_some() && prev_data.is_some() {
                    // check for button callback actions
                    for diff in prev_data.clone().unwrap().buttons.iter()
                                    .zip(data.clone().unwrap().buttons.iter())
                                    .zip(0_usize..) {
                                    // ((Action, Action), usize)
                        //on press
                        if (diff.0).0 == &Action::Release
                            && (diff.0).1 == &Action::Press {
                            match builder.on_press.get(&diff.1).unwrap_or(&None) {
                                &Some(ref f) => f(()),
                                &None => (),
                            };
                        }
                        //on release
                        if (diff.0).0 == &Action::Press
                            && (diff.0).1 == &Action::Release {
                            match builder.on_release.get(&diff.1).unwrap_or(&None) {
                                &Some(ref f) => f(()),
                                &None => (),
                            };
                        }
                    }
                    //check for movement callback actions
                    for diff in prev_data.clone().unwrap().axes.iter()
                                    .zip(data.clone().unwrap().axes.iter())
                                    .zip(0_usize..) {
                                    //((f64, f64), usize)
                        if (diff.0).0 != (diff.0).1 {
                            match builder.on_move.get(&diff.1).unwrap_or(&None) {
                                &Some(ref f) => f((diff.0).1.clone()),
                                &None => (),
                            }
                        }
                    }
                }
                prev_data = data;
                thread::sleep(builder.polling);
            }
            println!("thread is over");
            ()
        });
        JoystickThread {
            thread: joinhandle,
            running: running_clone,
        }
    }

}

impl JoystickThread {

    pub fn tear_down(self) -> Result<(), Box<Any + Send + 'static>> {
        *self.running.lock().unwrap() = false;
        self.thread.join()
    }

}

impl JoystickWrapper {
    fn get_data(&self) -> Result<JoystickData, JoystickError> {
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
            false => Err(JoystickError::NotPresent)
        }
    }
}
