# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2023-02-01

### Added

- `MaskEnableFlags` bitflags struct
- `set_critical_alert_latch(bool)` mutator method
- `set_warning_alert_latch(bool)` mutator method
- `get_channels_enabled(&[bool])` method
- `set_channels_enabled(&mut [bool])` mutator method

### Changed

- `read_alert_flags()` method now returns `MaskEnableFlags` bitflags struct
- `read_alert_flags()` method now allows preserving the alert flags
- Documentation

## [0.2.1] - 2023-02-01

### Fixed

- `set_mode(OperatingMode)` should now operate correctly
- Documentation

## [0.2.0] - 2023-02-01

### Added

- `Voltage` immutable struct
- `read_alert_flags()` mutator method
- `get_critical_limit(u8)` method
- `set_critical_limit(u8, Voltage)` mutator method
- `get_warning_limit(u8)` method
- `set_warning_limit(u8, Voltage)` mutator method
- `get_power_valid_limits()` method
- `set_power_valid_limits(Voltage, Voltage)` mutator method
- `get_mode()` method
- `set_mode(OperatingMode)` mutator method
- Inline documentation

### Changed

- `get_shunt_voltage(u8)` method now returns `Voltage` struct
- `get_bus_voltage(u8)` method now returns `Voltage` struct

### Removed

- `get_shunt_voltage_mv(u8)` method
- `get_bus_voltage_mv(u8)` method
- `get_shunt_voltage_uv(u8)` method
- `get_bus_voltage_uv(u8)` method

## [0.1.0] - 2023-01-31

### Added

- `reset()` mutator method
- `get_die_id()` method
- `get_manufacturer_id()` method
- `get_bus_voltage(u8)` method
- `get_bus_voltage_mv(u8)` method
- `get_shunt_voltage_uv(u8)` method
- `get_shunt_voltage_mv(u8)` method
- `set_channel_enabled(u8, bool)` mutator method
- `is_channel_enabled(u8)` method
- `get_configuration()` method
- `INA3221` driver structure for device access
