/// Represents the operating mode of the INA3221
#[derive(Debug)]
pub enum OperatingMode {
    /// Power save mode, no measurements are performed
    PowerDown = 0x00,
    /// Shunt and bus voltage measurements are performed once
    Triggered = 0x03,
    /// Shunt and bus voltage measurements are performed continuously
    Continuous = 0x07,
}
