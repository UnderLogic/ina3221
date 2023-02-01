# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
