extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derives the `FromRedisValue` trait for a struct
///
/// This procedural macro allows you to automatically implement the `FromRedisValue` trait
/// for a given struct, enabling seamless deserialization from Redis values. It is particularly
/// useful when working with Redis data stored in JSON format, as it simplifies the process of
/// converting Redis values into Rust structs.
///
/// # Requirements
///
/// Model must implement `serde::Serialize` and `serde::Deserialize`
///
/// # Arguments
///
/// * `input` - The input token stream representing the struct for which the trait is being derived.
///
/// # Returns
///
/// A `TokenStream` containing the implementation of the `FromRedisValue` trait for the specified struct.
///
/// # Examples
///
/// ```rust, no_run
/// // redis-rs, since it reimported in grapple_db, you can use it like this
/// use grapple_db::redis::{self, FromRedisValue};
/// use grapple_redis_macros::FromRedisValue;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, Serialize, FromRedisValue)]
/// struct MyStruct {
///     field1: String,
///     field2: i32,
/// }
///
/// fn example(redis_value: &redis::Value) -> Result<MyStruct, redis::RedisError> {
///     MyStruct::from_redis_value(redis_value)
/// }
/// ```
#[proc_macro_derive(FromRedisValue)]
pub fn from_redis_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl redis::FromRedisValue for #name {
            fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                let json_string: String = String::from_redis_value(v)?;

                let model: Self = serde_json::from_str(&json_string).map_err(|_| {
                    redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Failed to deserialize",
                    ))
                })?;
                Ok(model)
            }
        }
    };

    TokenStream::from(expanded)
}
