use core::fmt;

use lazy_static::lazy_static;
use libzynq_dev::uart::{DevUart, Uart};
use spin::{mutex::Mutex, Mutex};

lazy_static! {
    pub static ref UART_0: Mutex<Uart> = Mutex::new(Uart::new(0xE000_0000));
}

pub fn uart_write(args: &str) {
    let mut uart = UART_0.lock().write_str(args);
}
