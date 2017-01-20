extern crate glfw;

use std::any::Any;
use std::time::Duration;
use std::thread::JoinHandle;
use std::collections::hash_map::HashMap;

use glfw::{Glfw, Joystick, JoystickId, Action};

#[derive(Debug, Clone, PartialEq)]
pub struct JoystickData {
    name: String,
    axes: Vec<f32>,
    buttons: Vec<Action>,
}

type Callback = Option<Box<(Fn())>>;

pub struct JoystickThreadBuilder {
    handle: Joystick,
    on_press: HashMap<usize, Callback>,
    on_release: HashMap<usize, Callback>,
    on_hold: HashMap<usize, Callback>,
    on_move: HashMap<usize, Callback>,
    polling: Duration,
}

pub struct JoystickThread {
    //thread: JoinHandle<()>,
}

impl JoystickData {

}

impl JoystickThreadBuilder {

    pub fn new(glfw: Glfw, id: JoystickId, rate: Duration) -> Self {
        JoystickThreadBuilder  {
            handle: glfw.get_joystick(id),
            on_press: HashMap::new(),
            on_release: HashMap::new(),
            on_hold: HashMap::new(),
            on_move: HashMap::new(),
            polling: rate,
        }
    }

    pub fn on_press(&mut self, button: usize, callback: Callback) -> &mut Self {
        self.on_press.insert(button, callback);
        self
    }

    pub fn on_release(&mut self, button: usize, callback: Callback) -> &mut Self {
        self.on_release.insert(button, callback);
        self
    }

    pub fn on_hold(&mut self, button: usize, callback: Callback) -> &mut Self {
        self.on_hold.insert(button, callback);
        self
    }

    pub fn on_move(&mut self, axis: usize, callback: Callback) -> &mut Self {
        self.on_move.insert(axis, callback);
        self
    }

    pub fn spin_up(self) -> JoystickThread {
        JoystickThread {

        }
    }

}

impl JoystickThread {

    pub fn tear_down(self) -> Result<(), Box<Any + Send + 'static>> {
        Ok(())
    }
}
