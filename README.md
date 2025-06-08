# grapple_redis_macros

`grapple_redis_macros` is a procedural macro library designed to simplify the deserialization of Redis values into Rust structs for use with the `grapple_db` Redis Client. This library provides a convenient way to automatically implement the `FromRedisValue` trait for your structs, enabling seamless integration with Redis data stored in JSON format.

## Features

- **Automatic Trait Derivation**: Automatically derive the `FromRedisValue` trait for your structs, simplifying the process of deserializing Redis values.
- **Integration with Serde**: Leverage the power of Serde for serialization and deserialization, ensuring compatibility with JSON data.
- **Easy to Use**: Simplifies the process of converting Redis values into Rust structs with minimal boilerplate code.

## Requirements

To use this library, your model must implement the `serde::Serialize` and `serde::Deserialize` traits.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
grapple_redis_macros = "0.1.0"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
