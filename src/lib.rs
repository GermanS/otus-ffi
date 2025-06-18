#![allow(non_snake_case)]
use std::mem::transmute;

pub struct Args {
    power: u32,
    step: u32,
    state: bool,
}

pub struct Socket {
    power: u32,
    step: u32,
    state: bool,
}

impl Socket {
    pub fn new(args: &Args) -> Socket {
        Socket {
            power: args.power,
            step: args.step,
            state: args.state,
        }
    }

    pub fn get(&self) -> u32 {
        self.power
    }

    pub fn incr(&mut self) -> u32 {
        self.power += self.step;
        self.power
    }

    pub fn decr(&mut self) -> u32 {
        self.power -= self.step;
        self.power
    }

    pub fn state(&self) -> bool {
        self.state
    }

    pub fn turn(&mut self) -> bool {
        self.state = !self.state;
        self.state
    }
}

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_socket(args: &Args) -> *mut Socket {
    unsafe { transmute(Box::new(Socket::new(args))) }
}
/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_power(ptr: *mut Socket) -> u32 {
    let socket = unsafe { &mut *ptr };
    socket.get()
}

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn inc_power(ptr: *mut Socket) -> u32 {
    let socket = unsafe { &mut *ptr };
    socket.incr()
}
/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dec_power(ptr: *mut Socket) -> u32 {
    let socket = unsafe { &mut *ptr };
    socket.decr()
}

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn status(ptr: *mut Socket) -> bool {
    let socket = unsafe { &mut *ptr };
    socket.state()
}

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn turn(ptr: *mut Socket) -> bool {
    let socket = unsafe { &mut *ptr };
    socket.turn()
}
