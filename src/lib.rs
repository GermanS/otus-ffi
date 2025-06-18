#![allow(non_snake_case)]
use std::mem::transmute;

pub struct Args {
    init: u32,
    by: u32,
}

pub struct Counter {
    val: u32,
    by: u32,
}

impl Counter {
    pub fn new(args: &Args) -> Counter {
        Counter {
            val: args.init,
            by: args.by,
        }
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn incr(&mut self) -> u32 {
        self.val += self.by;
        self.val
    }

    pub fn decr(&mut self) -> u32 {
        self.val -= self.by;
        self.val
    }
}

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn createCounter(args: &Args) -> *mut Counter {
    unsafe { transmute(Box::new(Counter::new(args))) }
}
/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getCounterValue(ptr: *mut Counter) -> u32 {
    let counter = unsafe { &mut *ptr };
    counter.get()
}

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn incrementCounterBy(ptr: *mut Counter) -> u32 {
    let counter = unsafe { &mut *ptr };
    counter.incr()
}
/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn decrementCounterBy(ptr: *mut Counter) -> u32 {
    let counter = unsafe { &mut *ptr };
    counter.decr()
}
/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroyCounter(ptr: *mut Counter) {
    let _counter: Box<Counter> = unsafe { transmute(ptr) };
    // Drop
}
