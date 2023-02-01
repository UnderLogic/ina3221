use crate::registers::Register;
use crate::{helpers, OperatingMode, Voltage};
use core::cell::RefCell;
use hal::i2c::I2c;

const RESET_FLAG: u16 = 0x8000;
const CHANNEL_1_FLAG: u16 = 0x4000;
const CHANNEL_2_FLAG: u16 = 0x2000;
const CHANNEL_3_FLAG: u16 = 0x1000;

const SHUNT_VOLTAGE_SCALE_FACTOR: i32 = 40;
const BUS_VOLTAGE_SCALE_FACTOR: i32 = 8000;

/// Device driver for the INA3221 current and power monitor
///
/// The [INA3221] is a triple-channel shunt and bus voltage monitor that can be used to measure
/// current and power consumption from up to three loads.
///
/// It is very similar to the INA219, but with a few additional features and a different register
/// layout.
///
/// # Configuration
///
/// The INA3221 can be configured to use different operating modes and alert thresholds.
///
/// It is important to note that the INA3221 will retain configuration settings unless the device is
/// reset or power cycled. You can manually reset the device by calling the `reset()` method.
///
///
/// # Channels
///
/// The INA3221 has three channels, each of which can be used to measure the current and power
/// consumption of a load. The voltage should not exceed 26V and the maximum current draw should
/// not exceed 3.2A (per-channel).
///
/// The INA3221 can be configured to enable or disable each channel individually.
/// By default, all three channels are enabled.
///
/// The last measurements are retained in the device's registers, even if the channel
/// is disabled. This means that the last measurements will be returned when the channel is read.
///
/// Channel index is zero-based, so channel 1 is index 0, channel 2 is index 1, and channel 3 is index 2.
/// Attempting to use a channel index outside of the range of 0-2 will use the third channel.
///
/// # Shunt Resistor
///
/// The INA3221 requires a shunt resistor to be connected to each channel. The value of this resistor
/// is used to calculate the current draw from the load. The shunt resistor should be as small as
/// possible, but large enough to handle the maximum current draw of the load. The shunt resistor
/// should be connected between the load and the INA3221's shunt voltage measurement pin.
///
/// A common value for the shunt resistor is **0.1 ohms**, which can handle up to 3.2A of current draw.
///
/// # Current Calculation
///
/// Unlike the INA219, the INA3221 does not store the shunt resistor value in the device,
/// and so the current draw must be calculated manually instead of using the device's built-in
/// current calculation and register.
///
/// The bonus of this is that the shunt resistor value can be changed without the need to
/// calibrate the INA3221, only the firmware needs to be updated.
///
/// The current draw can be calculated using Ohm's Law:
/// I = V / R
///
/// It is important to be mindful of the units used when calculating the current draw.
///
/// ## Example
///
/// ```rust
/// // Assume a shunt resistor value of 0.1 ohms
/// let shunt_resistor = 0.1f32;
/// let shunt_voltage = ina.get_shunt_voltage(0).unwrap();
/// let current_milliamps = shunt_voltage.to_millivolts() / shunt_resistor;
/// ```
///
/// # Power Calculation
///
/// Similar to the current calculation, the power draw can be calculated using Ohm's Law:
/// P = I * V
///
/// Again, it is important to be mindful of the units used when calculating the power draw.
///
/// ## Example
///
/// ```rust
/// // Assume a shunt resistor value of 0.1 ohms
/// let shunt_resistor = 0.1f32;
/// let shunt_voltage = ina.get_shunt_voltage(0).unwrap();
/// let bus_voltage = ina.get_bus_voltage(0).unwrap();
/// let load_voltage = bus_voltage.add(&shunt_voltage);
///
/// let current_milliamps = shunt_voltage.to_millivolts() / shunt_resistor;
/// let power_milliwatts = current_milliamps * load_voltage.to_volts();
/// ```
///
/// # Operating Mode
///
/// The INA3221 can be configured to operate in one of three modes:
///
/// - Power-down
/// - Triggered
/// - Continuous
///
/// The default operating mode is continuous, which means that the device will continuously
/// measure the shunt and bus voltages and store the results in the device's registers.
/// Any disabled channels will not be measured and are skipped from the measurement cycle.
///
/// When triggered mode is set, the device will take a one-time measurement of the shunt and bus
/// voltages, and then enter a power-down state. The device will remain in this state until
/// the operating mode is changed to either continuous or triggered (again).
///
/// The power-down mode will disable all measurements and put the device into a low-power state.
/// The last measurement results will be stored in the device's registers and can be read even
/// while powered down.
///
/// # Alerts
/// The INA3221 can be configured to trigger various alerts based on the various measurements.
///
/// The following alerts are available:
///
/// - Over-current (Critical)
/// - Over-current (Warning)
/// - Under-voltage (PowerValid)
/// - Over-voltage (PowerValid)
///
/// See the [INA3221] datasheet for more information on the available alerts and
/// how they are triggered.
///
/// # Example
///
/// ```rust
/// use ina3221::Voltage;
///
/// let max_milliamps = 1000f32;    // 1A
/// let shunt_resistor = 0.1f32;    // 0.1 ohms
///
/// // Calculate the maximum voltage that can be measured on the shunt using Ohm's Law (V = I * R)
/// let max_millivolts = max_milliamps * shunt_resistor; // 100mV
///
/// // Set the critical alert limit for channel 1 to raise when exceeding 1A of current draw
/// ina.set_critical_alert_limit(0, Voltage::from_millivolts(max_millivolts)).unwrap();
/// ```
///
/// Note that these limits are based on the shunt voltage, **not** the load voltage.
///
/// [INA3221]: https://www.ti.com/lit/ds/symlink/ina3221.pdf
pub struct INA3221<I2C> {
    i2c: RefCell<I2C>,
    /// I2C address of the INA3221
    pub address: u8,
}

impl<I2C, E> INA3221<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Create a new INA3221 driver instance from an I2C peripheral on a specific address
    ///
    /// This is typically 0x40, 0x41, or 0x42 depending on the A0 pin setting
    pub fn new(i2c: I2C, address: u8) -> INA3221<I2C> {
        INA3221 {
            i2c: RefCell::new(i2c),
            address,
        }
    }

    /// Gets the active configuration bits from the INA3221
    pub fn get_configuration(&self) -> Result<u16, E> {
        self.read_register(Register::Configuration)
    }

    /// Gets the operating mode of the INA3221
    pub fn get_mode(&self) -> Result<OperatingMode, E> {
        let config = self.get_configuration()?;
        let mode = match config & 0x7 {
            0x01 => OperatingMode::Triggered,
            0x02 => OperatingMode::Triggered,
            0x03 => OperatingMode::Triggered,
            0x05 => OperatingMode::Continuous,
            0x06 => OperatingMode::Continuous,
            0x07 => OperatingMode::Continuous,
            _ => OperatingMode::PowerDown,
        };

        Ok(mode)
    }

    /// Sets the operating mode of the INA3221
    ///
    /// Setting the mode to `OperatingMode::Triggered` will trigger a measurement cycle
    pub fn set_mode(&mut self, mode: OperatingMode) -> Result<(), E> {
        let config = self.get_configuration()?;
        let new_config = (config & 0xFFF8) | mode as u16;
        self.write_register(Register::Configuration, new_config)
    }

    /// Gets the enabled status for all three channels, storing them in an array
    ///
    /// This is useful for iterating over all channels without having to call
    /// `is_channel_enabled` multiple times
    pub fn get_channels_enabled(&self, statuses: &mut [bool]) -> Result<(), E> {
        let config = self.get_configuration()?;
        statuses[0] = config & CHANNEL_1_FLAG > 0;
        statuses[1] = config & CHANNEL_2_FLAG > 0;
        statuses[2] = config & CHANNEL_3_FLAG > 0;
        Ok(())
    }

    /// Sets the enabled status for all three channels
    ///
    /// This is useful for enabling or disabling all channels at once without having to call
    /// `set_channel_enabled` multiple times
    ///
    /// Disabling a channel prevents it from being measured, but it can still be read
    /// for the last measurement result
    pub fn set_channels_enabled(&mut self, enabled: &[bool]) -> Result<(), E> {
        let config = self.get_configuration()?;
        let mut new_config = config & 0xFFF8;
        if enabled[0] {
            new_config |= CHANNEL_1_FLAG;
        }
        if enabled[1] {
            new_config |= CHANNEL_2_FLAG;
        }
        if enabled[2] {
            new_config |= CHANNEL_3_FLAG;
        }
        self.write_register(Register::Configuration, new_config)
    }

    /// Checks if a monitoring channel is enabled on the INA3221
    ///
    /// A disabled channel can still be read, but will not be measured until it is re-enabled
    pub fn is_channel_enabled(&self, channel: u8) -> Result<bool, E> {
        let flag = match channel {
            0 => CHANNEL_1_FLAG,
            1 => CHANNEL_2_FLAG,
            _ => CHANNEL_3_FLAG,
        };

        let config = self.get_configuration()?;
        Ok(config & flag > 0)
    }

    /// Enables or disables a monitoring channel on the INA3221
    ///
    /// Disabling a channel prevents it from being measured, but it can still be read
    /// for the last measurement result
    pub fn set_channel_enabled(&mut self, channel: u8, enabled: bool) -> Result<(), E> {
        let flag = match channel {
            0 => CHANNEL_1_FLAG,
            1 => CHANNEL_2_FLAG,
            _ => CHANNEL_3_FLAG,
        };

        let config = self.get_configuration()?;

        // Toggle the channel bit in the configuration
        match enabled {
            true => self.write_register(Register::Configuration, config | flag),
            false => self.write_register(Register::Configuration, config & !flag),
        }
    }

    /// Gets the shunt voltage of a specific monitoring channel
    pub fn get_shunt_voltage(&self, channel: u8) -> Result<Voltage, E> {
        let register = match channel {
            0 => Register::ShuntVoltage1,
            1 => Register::ShuntVoltage2,
            _ => Register::ShuntVoltage3,
        };

        // LSB = 40uV, meaning the value is downscaled 40:1
        let raw_value = self.read_register(register)?;
        let microvolts = helpers::convert_from_12bit_signed(raw_value) * SHUNT_VOLTAGE_SCALE_FACTOR;
        Ok(Voltage::from_microvolts(microvolts))
    }

    /// Gets the bus voltage of a specific monitoring channel
    pub fn get_bus_voltage(&self, channel: u8) -> Result<Voltage, E> {
        let register = match channel {
            0 => Register::BusVoltage1,
            1 => Register::BusVoltage2,
            _ => Register::BusVoltage3,
        };

        // LSB = 8mV (8000uV), meaning the value is downscaled 8:1
        let raw_value = self.read_register(register)?;
        let microvolts = helpers::convert_from_12bit_signed(raw_value) * BUS_VOLTAGE_SCALE_FACTOR;
        Ok(Voltage::from_microvolts(microvolts))
    }

    /// Gets the critical alert limit of a specific monitoring channel
    ///
    /// This is the shunt voltage limit that will trigger a critical alert on that channel
    pub fn get_critical_alert_limit(&self, channel: u8) -> Result<Voltage, E> {
        let register = match channel {
            0 => Register::CriticalAlertLimit1,
            1 => Register::CriticalAlertLimit2,
            _ => Register::CriticalAlertLimit3,
        };

        // LSB = 40uV, meaning the value is downscaled 40:1
        let raw_value = self.read_register(register)?;
        let microvolts = helpers::convert_from_12bit_signed(raw_value) * SHUNT_VOLTAGE_SCALE_FACTOR;
        Ok(Voltage::from_microvolts(microvolts))
    }

    /// Sets the critical alert limit for a specific monitoring channel
    ///
    /// This is the shunt voltage limit that will trigger a critical alert on that channel
    pub fn set_critical_alert_limit(
        &mut self,
        channel: u8,
        voltage_limit: Voltage,
    ) -> Result<(), E> {
        let register = match channel {
            0 => Register::CriticalAlertLimit1,
            1 => Register::CriticalAlertLimit2,
            _ => Register::CriticalAlertLimit3,
        };

        // LSB = 40uV, meaning the value is downscaled 40:1
        let raw_value = voltage_limit.to_microvolts() / SHUNT_VOLTAGE_SCALE_FACTOR;
        self.write_register(register, helpers::convert_to_12bit_signed(raw_value))
    }

    /// Gets the warning alert limit of a specific monitoring channel
    ///
    /// This is the shunt voltage limit that will trigger a warning alert on that channel
    pub fn get_warning_alert_limit(&self, channel: u8) -> Result<Voltage, E> {
        let register = match channel {
            0 => Register::WarningAlertLimit1,
            1 => Register::WarningAlertLimit2,
            _ => Register::WarningAlertLimit3,
        };

        // LSB = 40uV, meaning the value is downscaled 40:1
        let raw_value = self.read_register(register)?;
        let microvolts = helpers::convert_from_12bit_signed(raw_value) * SHUNT_VOLTAGE_SCALE_FACTOR;
        Ok(Voltage::from_microvolts(microvolts))
    }

    /// Sets the warning alert limit for a specific monitoring channel
    ///
    /// This is the shunt voltage limit that will trigger a warning alert on that channel
    pub fn set_warning_alert_limit(
        &mut self,
        channel: u8,
        voltage_limit: Voltage,
    ) -> Result<(), E> {
        let register = match channel {
            0 => Register::WarningAlertLimit1,
            1 => Register::WarningAlertLimit2,
            _ => Register::WarningAlertLimit3,
        };

        // LSB = 40uV, meaning the value is downscaled 40:1
        let raw_value = voltage_limit.to_microvolts() / SHUNT_VOLTAGE_SCALE_FACTOR;
        self.write_register(register, helpers::convert_to_12bit_signed(raw_value))
    }

    /// Gets the power valid limits of **all** enabled monitoring channels
    ///
    /// These are the lower and upper limits (respectively) for the bus voltage that will trigger
    /// a power valid alert on all enabled channels
    pub fn get_power_valid_limits(&self) -> Result<(Voltage, Voltage), E> {
        // LSB = 8mV (8000uV), meaning the value is downscaled 8:1
        let lower_raw_value = self.read_register(Register::PowerValidLowerLimit)?;
        let upper_raw_value = self.read_register(Register::PowerValidUpperLimit)?;

        let lower_microvolts =
            helpers::convert_from_12bit_signed(lower_raw_value) * BUS_VOLTAGE_SCALE_FACTOR;
        let upper_microvolts =
            helpers::convert_from_12bit_signed(upper_raw_value) * BUS_VOLTAGE_SCALE_FACTOR;

        Ok((
            Voltage::from_microvolts(lower_microvolts),
            Voltage::from_microvolts(upper_microvolts),
        ))
    }

    /// Sets the power valid limits for **all** enabled monitoring channels
    ///
    /// These are the upper and lower limits for the bus voltage that will trigger a power valid alert
    /// on all enabled channels
    pub fn set_power_valid_limits(
        &mut self,
        lower_limit: Voltage,
        upper_limit: Voltage,
    ) -> Result<(), E> {
        // LSB = 8mV (8000uV), meaning the value is downscaled 8:1
        let lower_raw_value = lower_limit.to_microvolts() / BUS_VOLTAGE_SCALE_FACTOR;
        let upper_raw_value = upper_limit.to_microvolts() / BUS_VOLTAGE_SCALE_FACTOR;

        self.write_register(
            Register::PowerValidLowerLimit,
            helpers::convert_to_12bit_signed(lower_raw_value),
        )?;

        self.write_register(
            Register::PowerValidUpperLimit,
            helpers::convert_to_12bit_signed(upper_raw_value),
        )?;

        Ok(())
    }

    /// Reads the alert flags from the INA3221, clearing them upon read
    ///
    /// The flags are returned as a bitfield, see the datasheet for more information
    pub fn read_alert_flags(&mut self) -> Result<u16, E> {
        self.read_register(Register::MaskEnable)
    }

    /// Gets the manufacturer ID from the INA3221
    ///
    /// This value is always 0x5449 ('TI' in ASCII), or at least should be for genuine INA3221s
    pub fn get_manufacturer_id(&self) -> Result<u16, E> {
        self.read_register(Register::ManufacturerId)
    }

    /// Gets the die ID from the INA3221
    ///
    /// This value is always 0x3220, or at least should be for genuine INA3221s
    pub fn get_die_id(&self) -> Result<u16, E> {
        self.read_register(Register::DieId)
    }

    /// Resets the INA3221
    ///
    /// This clears all configuration bits and sets the default configuration
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
}
