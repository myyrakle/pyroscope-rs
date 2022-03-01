use crate::PyroscopeAgent;
use pyroscope_backends::pyspy::Pyspy;
use pyroscope_backends::rbspy::Rbspy;

use std::sync::{Arc, Mutex};

pub static RBSPY: Option<Arc<Mutex<bool>>> = None;

#[link(name = "pyroscope_ffi", vers = "0.1")]
#[no_mangle]
pub fn add(x: i32, y: i32) -> i32 {
    x + y + 5
}

#[link(name = "pyroscope_ffi", vers = "0.1")]
#[no_mangle]
pub fn initialize_agent(pid: i32) -> bool {
    std::thread::spawn(move || {
        let mut agent = PyroscopeAgent::builder("http://localhost:4040", "rbspy.basic")
            .backend(Rbspy::new(pid))
            .build()
            .unwrap();

        agent.start();

        // should instead block here until a stop signal is received
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000000));
        }
    });

    true
}

#[link(name = "pyroscope_ffi", vers = "0.1")]
#[no_mangle]
pub fn initialize_python(pid: i32) -> bool {
    std::thread::spawn(move || {
        let mut agent = PyroscopeAgent::builder("http://localhost:4040", "python.basic")
            .backend(Pyspy::new(pid))
            .build()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5000));

        agent.start();

        // should instead block here until a stop signal is received
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000000));
        }
    });

    true
}
#[link(name = "pyroscope_ffi", vers = "0.1")]
#[no_mangle]
pub fn safe_mutex() -> bool {
    // set RBSPY to true
    let a = RBSPY.as_ref().take();

    true
}

#[link(name = "pyroscope_ffi", vers = "0.1")]
#[no_mangle]
pub fn read_mutex() -> bool {
    dbg!(RBSPY.as_ref().unwrap().lock().unwrap());

    true
}

#[no_mangle]
pub extern "C" fn double(x: i32) -> i32 {
    x * 2
}