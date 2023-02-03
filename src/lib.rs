//! Embedded driver for the INA3221 current and power monitor
//!
//! Provides a platform agnostic driver for the [INA3221] triple-channel current and power monitor
//! that can be used with any [embedded-hal] v1.0 blocking I2C implementation.
//!
//! [INA3221]: https://www.ti.com/lit/ds/symlink/ina3221.pdf
//! [embedded-hal]: https://docs.rs/embedded-hal/1.0.0/embedded_hal/
#![no_std]
extern crate embedded_hal as hal;

mod driver;
mod flags;
mod helpers;
mod mode;
mod registers;

pub use driver::INA3221;
pub use flags::MaskEnableFlags;
pub use mode::OperatingMode;
pub use ohms::*;
