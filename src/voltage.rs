/// Voltage is a struct that represents a voltage in microvolts
/// and provides methods to convert to and from millivolts and volts
///
/// This avoids needing to create various function signatures for each desired unit
///
/// The struct is immutable and provides methods to create a new instance
#[derive(Debug, Copy, Clone)]
pub struct Voltage {
    microvolts: i32,
}

impl Voltage {
    /// Creates a new Voltage struct from a value in microvolts
    pub fn from_microvolts(microvolts: i32) -> Self {
        Voltage { microvolts }
    }

    /// Creates a new Voltage struct from a value in millivolts
    pub fn from_millivolts(millivolts: f32) -> Self {
        Voltage {
            microvolts: (millivolts * 1000f32) as i32,
        }
    }

    /// Creates a new Voltage struct from a value in volts
    pub fn from_volts(volts: f32) -> Self {
        Voltage {
            microvolts: (volts * 1000000f32) as i32,
        }
    }

    /// Returns the value in microvolts
    pub fn to_microvolts(&self) -> i32 {
        self.microvolts
    }

    /// Returns the value in millivolts
    pub fn to_millivolts(&self) -> f32 {
        self.microvolts as f32 / 1000f32
    }

    /// Returns the value in volts
    pub fn to_volts(&self) -> f32 {
        self.microvolts as f32 / 1000000f32
    }

    /// Returns whether the voltage is negative
    pub fn is_negative(&self) -> bool {
        self.microvolts < 0
    }

    /// Returns whether the voltage is positive
    pub fn is_positive(&self) -> bool {
        self.microvolts > 0
    }

    /// Returns whether the voltage is zero
    pub fn is_zero(&self) -> bool {
        self.microvolts == 0
    }

    /// Returns the absolute value of the voltage
    pub fn abs(&self) -> Self {
        Voltage {
            microvolts: self.microvolts.abs(),
        }
    }

    /// Returns the inverted value of the voltage
    pub fn invert(&self) -> Self {
        Voltage {
            microvolts: -self.microvolts,
        }
    }

    /// Returns the sum of the two voltages
    pub fn add(&self, other: &Self) -> Self {
        Voltage {
            microvolts: self.microvolts + other.microvolts,
        }
    }

    /// Returns a voltage clamped between the min and max microvolt values
    pub fn clamp(&self, min_microvolts: i32, max_microvolts: i32) -> Self {
        Voltage {
            microvolts: self.microvolts.max(min_microvolts).min(max_microvolts),
        }
    }
}
