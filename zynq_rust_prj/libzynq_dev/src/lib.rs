#![no_std]

pub mod uart;

/// The trait is used for all device drivers
///
/// Any device driver must implement this trait
pub trait Device {
    /// Get the base address of the device
    fn get_base_addr(&self) -> usize;

    /// Initialize the device
    fn init(&self);

    /// New up the device
    fn new(base_addr: usize) -> Self;
}
