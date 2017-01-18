extern crate glfw;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use glfw::{Joystick, JoystickId, Action};

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not load GLFW!");
    let joystick = JoystickHandle::new(glfw.get_joystick(JoystickId::Joystick1));
    let joystick_thread = joystick.spin_up();
    loop {
        let data = joystick_thread.rx.recv().unwrap();
        match data {
            Ok(d) => {
                println!("{:?}", &d);
                if d.buttons[0] == Action::Press {
                    println!("Pressed Button 0, exit!");
                    joystick_thread.tear_down().unwrap();
                    break;
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                joystick_thread.tear_down().unwrap();
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}

#[derive(Clone, Copy)]
struct JoystickHandle {
    inner: Joystick,
}

struct JoystickThreadHandle {
    thread: JoinHandle<bool>,
    rx: Receiver<Result<JoystickData, String>>,
    running: Arc<Mutex<bool>>
}

#[derive(Debug, Clone)]
struct JoystickData {
    name: String,
    axes: Vec<f32>,
    buttons: Vec<Action>,
}

impl JoystickHandle {

    fn new(handle: Joystick) -> Self {
        JoystickHandle {
            inner: handle,
        }
    }

    fn get_data(&self) -> Result<JoystickData, String> {
        match self.inner.is_present() {
            true => Ok(JoystickData {
                    name: self.inner.get_name(),
                    axes: self.inner.get_axes(),
                    buttons: self.inner.get_buttons()
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

    fn spin_up(self) -> JoystickThreadHandle {
        let (tx, rx) = mpsc::channel();
        let running_orig = Arc::new(Mutex::new(true));
        let running = running_orig.clone();
        let joinhandle = thread::spawn(move || {
            while *running.lock().unwrap() {
                tx.send(self.get_data()).unwrap();
                thread::sleep(time::Duration::from_millis(5));
            }
            false
        });
        JoystickThreadHandle {
            thread: joinhandle,
            rx: rx,
            running: running_orig
        }
    }
}

impl JoystickThreadHandle {
    fn tear_down(self) -> std::result::Result<bool, Box<std::any::Any + Send + 'static>> {
        *self.running.lock().unwrap() = false;
        self.thread.join()
    }

}
