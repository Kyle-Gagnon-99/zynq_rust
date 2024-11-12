use crate::Device;

pub mod control_register;

impl Device for Uart {
    fn get_base_addr(&self) -> usize {
        self.base_addr
    }

    fn init(&self) {
        unsafe {
            // Reset the UART
            // Make sure everything is disabled
            core::ptr::write_volatile((self.base_addr + 0x00) as *mut u32, 0x0000_0000);

            // Set the baud rate generator
            core::ptr::write_volatile((self.base_addr + 0x18) as *mut u32, 4);
            // Set the baud rate divider
            core::ptr::write_volatile((self.base_addr + 0x34) as *mut u32, 216);

            // Set the mode (8N1, no parity, etc., default is usually fine)
            core::ptr::write_volatile((self.base_addr + 0x04) as *mut u32, 0x20);

            // Enable TX and RX in the control register
            core::ptr::write_volatile((self.base_addr + 0x00) as *mut u32, 0x00000014);
        }
    }

    fn new(base_addr: usize) -> Self {
        Uart { base_addr }
    }
}

/// The trait is used for all UART devices
pub trait DevUart: Device {
    fn write_str(&self, data: &str) {
        for byte in data.bytes() {
            self.write(byte);
        }
    }

    fn write(&self, data: u8) {
        unsafe {
            // Wait until the TX FIFO is empty
            while core::ptr::read_volatile((self.get_base_addr() + 0x2C) as *const u32) & 0x08 == 0
            {
            }

            // Write the byte to the TX FIFO
            core::ptr::write_volatile((self.get_base_addr() + 0x30) as *mut u8, data);
        }
    }
}

pub struct Uart {
    base_addr: usize,
}

impl DevUart for Uart {}

pub fn uart_init() -> Uart {
    let uart = Uart {
        base_addr: 0xE000_0000,
    };
    uart.init();
    uart
}
