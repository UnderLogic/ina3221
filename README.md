# ina3221
![crates.io](https://img.shields.io/crates/v/ina3221.svg)

Embedded driver for the [INA3221](https://www.ti.com/lit/ds/symlink/ina3221.pdf) triple-channel power monitor in Rust.

The INA3221 is very similar to the classic [INA219](https://www.ti.com/lit/ds/symlink/ina219.pdf) power monitor IC.

## Compatibility

Any board that supports [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) [blocking 1.0](https://docs.rs/embedded-hal/1.0.0-alpha.9/embedded_hal/index.html) `I2c` should be compatible with this library.

**NOTE:** Some HALs require feature flagging to enable 1.0 functionality, for example `esp-hal` requires the `eh1` feature.

## Installation

You can add via [crates.io](https://crates.io/):

```
$ cargo add ina3221
```

**NOTE:** Some HALs require feature flagging to enable 1.0 functionality, for example `esp-hal` requires the `eh1` feature.

## Example

This example assumes a **0.1 Ohm** shunt resistor for current and power calculations.

```rust
const INA3221_I2C_ADDR: u8 = 0x40;
const SHUNT_RESISTANCE: f32 = 0.1f32;

use ina3221::INA3221;

fn main() {
    let i2c = I2C::new(/* initialize your I2C here */);
    let ina = INA3221::new(i2c, INA3221_I2C_ADDR);

    let mut delay = Delay::new(/* initialize your delay/clocks */);

    loop {
        for channel in 0..3 {
            let shunt_mv = ina.get_shunt_voltage_mv(channel).unwrap();
            let bus_mv = ina.get_bus_voltage_mv(channel).unwrap();
            let load_voltage = (bus_mv + shunt_mv) / 1000f32;

            // Skip channel if no voltage present
            if shunt_mv == 0f32 {
                continue;
            }
            
            // Use Ohm's Law to calculate current and power with known resistance
            let current_milliamps = shunt_mv / SHUNT_RESISTANCE;
            let power_milliwatts = current_milliamps * load_voltage;
            
            println!(
                "Channel {}: load = {:.3} V, current = {:.3} mA, power = {:.3} mW",
                channel_index + 1,
                load_voltage,
                current_milliamps,
                power_milliwatts,
            );
        }
        
        delay.delay_ms(1000u32);
    }
}
```

### Output

This is sample output powering an Arduino Uno R3 over USB, running the blinky script.

```shell
Channel 1: load = 5.212 V, current = 36.800 mA, power = 191.790 mW
Channel 1: load = 5.211 V, current = 33.600 mA, power = 175.102 mW
Channel 1: load = 5.212 V, current = 36.800 mA, power = 191.790 mW
Channel 1: load = 5.219 V, current = 34.000 mA, power = 177.460 mW
Channel 1: load = 5.212 V, current = 36.800 mA, power = 191.790 mW
Channel 1: load = 5.211 V, current = 34.000 mA, power = 177.188 mW
Channel 1: load = 5.211 V, current = 34.000 mA, power = 177.188 mW
Channel 1: load = 5.212 V, current = 36.400 mA, power = 189.704 mW
Channel 1: load = 5.211 V, current = 34.000 mA, power = 177.188 mW
Channel 1: load = 5.212 V, current = 36.800 mA, power = 191.790 mW
Channel 1: load = 5.211 V, current = 34.000 mA, power = 177.188 mW
```
