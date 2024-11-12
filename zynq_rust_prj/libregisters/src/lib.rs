#![no_std]

/// The trait is used to define a register.
/// The register provides a register address that all subsequent operations will be performed on.
pub trait Register {
    /// Get the register address
    fn get_reg_addr(&self) -> usize;

    /// Get the register offset
    fn get_reg_offset(&self) -> usize;

    /// Get the register size
    fn get_reg_size(&self) -> usize;
}

/// The trait is used by registers that can be read from.
/// The read operation is performed on the register address.
pub trait RegisterR: Register {
    type R: Sized;

    /// Reads the register
    fn read(&self) -> Self::R {
        unsafe { core::ptr::read_volatile(self.get_reg_addr() as *const Self::R) }
    }
}

/// The trait is used by registers that can be written to.
/// The write operation is performed on the register address.
pub trait RegisterW: Register {
    type W: Sized;

    /// Writes to the register
    fn write(&self, value: Self::W) {
        unsafe { core::ptr::write_volatile(self.get_reg_addr() as *mut Self::W, value) }
    }

    /// Zeroes the register
    fn zero(&self) {}
}

/// The trait is used by registers that can be read from and written to.
/// The read and write operations are performed on the register address.
pub trait RegisterRW: RegisterR + RegisterW {
    fn modify<F>(&self, f: F)
    where
        F: FnOnce(<Self as RegisterR>::R) -> <Self as RegisterW>::W,
    {
        // Read the current value from the register
        let current_value = self.read();

        // Apply the closure to get the modified value
        let modified_value = f(current_value);

        // Write the modified value back to the register
        self.write(modified_value);
    }
}

#[macro_export]
macro_rules! define_register {
    ($peripheral:ident, $reg_name:ident, $reg_method_name:ident, $offset:expr, $reg_size:expr, R) => {
        pub struct $reg_name {
            base_addr: usize,
            offset: usize,
            reg_size: usize,
        }

        impl $crate::Register for $reg_name {
            fn get_reg_addr(&self) -> usize {
                self.base_addr
            }

            fn get_reg_offset(&self) -> usize {
                self.offset
            }

            fn get_reg_size(&self) -> usize {
                self.reg_size
            }
        }

        impl $crate::RegisterR for $reg_name {
            type R = R;
        }

        impl $reg_name {
            pub fn new(base_addr: usize, offset: usize, reg_size: usize) -> Self {
                $reg_name {
                    base_addr,
                    offset,
                    reg_size,
                }
            }
        }

        impl $peripheral {
            pub fn $reg_method_name(&self) -> $reg_name {
                $reg_name::new(self.get_base_addr(), $offset, $reg_size)
            }
        }
    };

    ($peripheral:ident, $reg_name:ident, $reg_method_name:ident, $offset:expr, $reg_size:expr, W) => {
        pub struct $reg_name {
            base_addr: usize,
            offset: usize,
            reg_size: usize,
        }

        impl $crate::Register for $reg_name {
            fn get_reg_addr(&self) -> usize {
                self.base_addr
            }

            fn get_reg_offset(&self) -> usize {
                self.offset
            }

            fn get_reg_size(&self) -> usize {
                self.reg_size
            }
        }

        impl $crate::RegisterR for $reg_name {
            type R = R;
        }

        impl $reg_name {
            pub fn new(base_addr: usize, offset: usize, reg_size: usize) -> Self {
                $reg_name {
                    base_addr,
                    offset,
                    reg_size,
                }
            }
        }

        impl $peripheral {
            pub fn $reg_method_name(&self) -> $reg_name {
                $reg_name::new(self.get_base_addr(), $offset, $reg_size)
            }
        }
    };

    ($peripheral:ident, $reg_name:ident, $reg_method_name:ident, $offset:expr, $reg_size:expr, $reg_ty:ty, RW) => {
        pub struct $reg_name {
            base_addr: usize,
            offset: usize,
            reg_size: usize,
        }

        impl $crate::Register for $reg_name {
            fn get_reg_addr(&self) -> usize {
                self.base_addr
            }

            fn get_reg_offset(&self) -> usize {
                self.offset
            }

            fn get_reg_size(&self) -> usize {
                self.reg_size
            }
        }

        impl $crate::RegisterR for $reg_name {
            type R = $reg_ty;
        }

        impl $crate::RegisterW for $reg_name {
            type W = $reg_ty;
        }

        impl $reg_name {
            pub fn new(base_addr: usize, offset: usize, reg_size: usize) -> Self {
                $reg_name {
                    base_addr,
                    offset,
                    reg_size,
                }
            }
        }

        impl $peripheral {
            pub fn $reg_method_name(&self) -> $reg_name {
                $reg_name::new(self.get_base_addr(), $offset, $reg_size)
            }
        }
    };
}
