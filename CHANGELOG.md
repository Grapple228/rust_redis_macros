# Changelog for grapple_db

## [0.2.0] - 09 June 2025

This change simplifies the macro crate's dependencies, as it primarily provides macro implementations and does not need to directly interact with Redis at compile time. The macro now relies on any library that re-exports the redis-rs library, enhancing flexibility and reducing complexity in the dependency graph.

### Removed

- Dependencies `serde`, `serde_json` and `deadpool-redis` are removed

### License

- This version is licensed under the MIT License.

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
