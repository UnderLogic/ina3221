use bitflags::bitflags;

bitflags! {
    /// Represents status and configuration flags for alerts
    pub struct MaskEnableFlags: u16 {
        /// The last measurement has been completed and the data is ready to be read
        const CONVERSION_READY = 0x01;
        /// Corresponds to the TimingControl pin, can be manually asserted by software
        const TIMING_CONTROL_ALERT = 0x02;
        /// Corresponds to the PowerValid pin, can be manually asserted by software
        const POWER_VALID_ALERT = 0x04;
        /// Channel 1 has exceeded the warning alert limit
        const WARNING_ALERT_1 = 0x08;
        /// Channel 2 has exceeded the warning alert limit
        const WARNING_ALERT_2 = 0x10;
        /// Channel 3 has exceeded the warning alert limit
        const WARNING_ALERT_3 = 0x20;
        /// The sum of the shunt voltages has exceeded the summation alert limit
        const SUMMATION_ALERT = 0x40;
        /// Channel 1 has exceeded the critical alert limit
        const CRITICAL_ALERT_1 = 0x80;
        /// Channel 2 has exceeded the critical alert limit
        const CRITICAL_ALERT_2 = 0x100;
        /// Channel 3 has exceeded the critical alert limit
        const CRITICAL_ALERT_3 = 0x200;
        /// Critical alert latch  enable, if set, the corresponding critical alert pin will be latched
        const CRITICAL_ALERT_LATCH = 0x400;
        /// Warning alert latch enable, if set, the corresponding warning alert pin will be latched
        const WARNING_ALERT_LATCH = 0x800;
        /// Include channel 1 in the summation calculation and stored in the shunt voltage summation register
        const SUMMATION_CONTROL_1 = 0x1000;
        /// Include channel 2 in the summation calculation and stored in the shunt voltage summation register
        const SUMMATION_CONTROL_2 = 0x2000;
        /// Include channel 3 in the summation calculation and stored in the shunt voltage summation register
        const SUMMATION_CONTROL_3 = 0x4000;
    }
}
