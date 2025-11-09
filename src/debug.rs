use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref DEBUG_COMMAND: Mutex<Option<Box<dyn Fn(String) + Send>>> = Mutex::new(None);
    static ref DEBUG_RESPONSE: Mutex<Option<Box<dyn Fn(String) + Send>>> = Mutex::new(None);
}

pub unsafe fn debug_command_via(f: impl Fn(String) + Send + 'static) {
    let mut command = DEBUG_COMMAND.lock().unwrap();
    *command = Some(Box::new(f));
}

pub unsafe fn debug_response_via(f: impl Fn(String) + Send + 'static) {
    let mut response = DEBUG_RESPONSE.lock().unwrap();
    *response = Some(Box::new(f));
}

pub unsafe fn dont_debug_command() {
    let mut command = DEBUG_COMMAND.lock().unwrap();
    *command = None;
}

pub unsafe fn dont_debug_response() {
    let mut response = DEBUG_RESPONSE.lock().unwrap();
    *response = None;
}

pub(crate) unsafe fn debug_command(s: String) -> bool {
    let command = DEBUG_COMMAND.lock().unwrap();
    command.as_ref().map(|f| f(s)).is_some()
}

pub(crate) unsafe fn debug_response(s: String) -> bool {
    let response = DEBUG_RESPONSE.lock().unwrap();
    response.as_ref().map(|f| f(s)).is_some()
}

pub unsafe fn toggle_debug_command() {
    let mut command = DEBUG_COMMAND.lock().unwrap();
    match *command {
        None => {
            println!("Command debug mode is ON");
            debug_command_via(|s| print!("{}", s))
        }
        Some(_) => {
            println!("Command debug mode is OFF");
            *command = None;
        }
    }
}

pub unsafe fn toggle_debug_response() {
    match DEBUG_RESPONSE.lock().unwrap().as_ref() {
        None => {
            println!("Response debug mode is ON");
            debug_response_via(|s| print!("{}", s))
        }
        Some(_) => {
            println!("Response debug mode is OFF");
            *DEBUG_RESPONSE.lock().unwrap() = None
        }
    }
}
