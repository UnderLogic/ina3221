use crate::registers::Register;
use core::cell::RefCell;
use hal::i2c::I2c;

const RESET_FLAG: u16 = 0x8000;

pub struct INA3221<I2C> {
    i2c: RefCell<I2C>,
    pub address: u8,
}

impl<I2C, E> INA3221<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> INA3221<I2C> {
        INA3221 {
            i2c: RefCell::new(i2c),
            address,
        }
    }

    pub fn read_configuration(&self) -> Result<u16, E> {
        self.read_register(Register::Configuration)
    }

    pub fn get_shunt_voltage_mv(&self, channel: u8) -> Result<f32, E> {
        let microvolts = self.get_shunt_voltage_uv(channel)?;
        Ok(microvolts as f32 / 1000f32)
    }

    pub fn get_shunt_voltage_uv(&self, channel: u8) -> Result<i32, E> {
        let register = match channel {
            0 => Register::ShuntVoltage1,
            1 => Register::ShuntVoltage2,
            _ => Register::ShuntVoltage3,
        };

        let raw_value = self.read_register(register)?;
        let microvolts = Self::convert_to_12bit_signed(raw_value) * 40;
        Ok(microvolts)
    }

    pub fn reset(&mut self) -> Result<(), E> {
        let config = self.read_register(Register::Configuration)?;
        self.write_register(Register::Configuration, config | RESET_FLAG)
    }

    fn select_register(&self, register: Register) -> Result<(), E> {
        self.i2c.borrow_mut().write(self.address, &[register as u8])
    }

    fn read_register(&self, register: Register) -> Result<u16, E> {
        self.select_register(register)?;

        let mut buffer: [u8; 2] = [0x00; 2];
        self.i2c.borrow_mut().read(self.address, &mut buffer)?;

        // Convert from big endian 16-bit word
        let word = ((buffer[0] as u16) << 8) + buffer[1] as u16;
        Ok(word)
    }

    fn write_register(&mut self, register: Register, value: u16) -> Result<(), E> {
        // Convert from little endian to big endian
        let msb = ((value >> 8) & 0xFF) as u8;
        let lsb = (value & 0xFF) as u8;

        let buffer: [u8; 3] = [register as u8, msb, lsb];
        self.i2c.borrow_mut().write(self.address, &buffer)?;
        Ok(())
    }

    fn convert_to_12bit_signed(value: u16) -> i32 {
        let value = match value & 0x8000 > 0 {
            true => !value + 1,
            false => value,
        };

        (value >> 3) as i32
    }
}
