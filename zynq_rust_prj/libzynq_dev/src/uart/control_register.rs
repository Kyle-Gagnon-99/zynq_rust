use super::Uart;
use libregisters::define_register;

define_register!(Uart, ControlRegister, control_register, 0x04, 4, u32, RW);
