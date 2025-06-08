# Changelog for grapple_db

## [0.1.0] - 08 June 2025

### Added

- Initial release of `grapple_redis_macros`.
- Implemented the `FromRedisValue` procedural macro for automatic deserialization of Redis values into Rust structs.
- Included support for Serde serialization and deserialization.
- Provided examples demonstrating the usage of the macro.

### Requirements

- Models must implement `serde::Serialize` and `serde::Deserialize` to work with the `FromRedisValue` macro.

### License

- This version is licensed under the MIT License.
