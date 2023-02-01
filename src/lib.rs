#![no_std]
#![allow(dead_code)]
extern crate embedded_hal as hal;

mod driver;
mod registers;

pub use driver::INA3221;
