# uuid-extra

Minimalist Rust crate providing UUID (v4 and v7) generation with Base58 and Base64 (standard, URL-safe, URL-safe no-pad) encoding helpers.

Designed for simplicity and ease of use when you need encoded UUIDs without much fuss.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
uuid-extra = "0.0.1" # Replace with the latest version
```

### Generate Base58 encoded UUID v7

```rust
use uuid_extra::new_v7_b58;

fn main() {
    let b58_uuid = new_v7_b58();
    println!("Base58 UUID v7: {}", b58_uuid);
    // Example output: B1ZwvMXbQ2iQjJWC9vA7Tf
}
```

### Generate Base64 URL-safe (no padding) encoded UUID v7

```rust
use uuid_extra::new_v7_b64url_nopad;

fn main() {
    let b64url_nopad_uuid = new_v7_b64url_nopad();
    println!("Base64 URL-safe (no-pad) UUID v7: {}", b64url_nopad_uuid);
    // Example output: MDE4Zjc5ZTctN2RhZS03Yj
}
```

## Features

-   Generate UUID v4 and v7.
-   Encode UUIDs to:
    -   Base58 (`new_v4_b58`, `new_v7_b58`)
    -   Base64 standard (`new_v4_b64`, `new_v7_b64`)
    -   Base64 URL-safe with padding (`new_v4_b64url`, `new_v7_b64url`)
    -   Base64 URL-safe without padding (`new_v4_b64url_nopad`, `new_v7_b64url_nopad`)
-   Decode from Base58/Base64 encoded strings back to `uuid::Uuid`.
    -   `from_b58(s: &str) -> Result<Uuid>`
    -   `from_b64(s: &str) -> Result<Uuid>`
    -   `from_b64url(s: &str) -> Result<Uuid>`
    -   `from_b64url_nopad(s: &str) -> Result<Uuid>`
-   Extract timestamp (milliseconds since epoch) from UUID v7:
    -   `to_time_epoch_ms(uuid: &Uuid) -> Result<i64>`

## Error Handling

The crate uses a simple `Result<T>` type alias (`crate::Result<T>`) with a custom `crate::Error` enum for error handling. This makes it straightforward to handle potential issues like decoding errors.

## Examples

For more detailed examples, please check the unit tests within each module (e.g., `src/extra_base58.rs`, `src/extra_base64.rs`).

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License.
