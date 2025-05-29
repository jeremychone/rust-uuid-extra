# `uuid-extra`

`uuid-extra` is a Rust crate that extends the functionality of the standard `uuid` crate by providing convenient methods for generating UUIDs (v4 and v7) and encoding/decoding them into various string formats, primarily Base58 and Base64 (including URL-safe variants). It also offers a utility to extract the timestamp from UUID v7.

This crate aims to simplify common tasks involving UUIDs where a compact or URL-friendly string representation is needed.

## Core Functionalities

-   **UUID Generation**:
    -   Generate version 4 (random) UUIDs.
    -   Generate version 7 (time-ordered) UUIDs.

-   **Base58 Encoding/Decoding**:
    -   Encode UUIDs into Base58 strings.
    -   Decode Base58 strings back into UUIDs.

-   **Base64 Encoding/Decoding**:
    -   Encode UUIDs into standard Base64 strings.
    -   Encode UUIDs into URL-safe Base64 strings (with and without padding).
    -   Decode these Base64 variants back into UUIDs.

-   **Timestamp Extraction**:
    -   Extract the Unix timestamp (in milliseconds) from a version 7 UUID.

## Usage

First, add `uuid-extra` to your `Cargo.toml`:

```toml
[dependencies]
uuid-extra = "0.0.1" # Or the latest version
```

### Generating UUIDs and Encoding

#### Base58

```rust
use uuid_extra::{new_v4_b58, new_v7_b58, from_b58};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new v4 UUID and get its Base58 representation
    let v4_b58_string = new_v4_b58();
    println!("UUID v4 Base58: {}", v4_b58_string);

    // Generate a new v7 UUID and get its Base58 representation
    let v7_b58_string = new_v7_b58();
    println!("UUID v7 Base58: {}", v7_b58_string);

    // Decode a Base58 string back to UUID
    let decoded_uuid: Uuid = from_b58(&v7_b58_string)?;
    println!("Decoded UUID from Base58: {}", decoded_uuid);

    Ok(())
}
```

#### Base64 (URL-safe, no padding)

This is often preferred for web applications.

```rust
use uuid_extra::{new_v4_b64url_nopad, new_v7_b64url_nopad, from_b64url_nopad};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new v4 UUID and get its URL-safe Base64 (no padding) representation
    let v4_b64url_nopad_string = new_v4_b64url_nopad();
    println!("UUID v4 Base64 URL (no pad): {}", v4_b64url_nopad_string);

    // Generate a new v7 UUID and get its URL-safe Base64 (no padding) representation
    let v7_b64url_nopad_string = new_v7_b64url_nopad();
    println!("UUID v7 Base64 URL (no pad): {}", v7_b64url_nopad_string);

    // Decode a URL-safe Base64 (no padding) string back to UUID
    let decoded_uuid: Uuid = from_b64url_nopad(&v7_b64url_nopad_string)?;
    println!("Decoded UUID from Base64 URL (no pad): {}", decoded_uuid);

    Ok(())
}
```

Other Base64 variants are also available:
-   `new_v4_b64()`, `new_v7_b64()`: Standard Base64 with padding.
-   `new_v4_b64url()`, `new_v7_b64url()`: URL-safe Base64 with padding.
-   `from_b64()`, `from_b64url()`: Corresponding decode functions.


### Extracting Timestamp from UUID v7

```rust
use uuid_extra::{new_v7, to_time_epoch_ms};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid_v7 = new_v7();
    println!("Generated UUID v7: {}", uuid_v7);

    let timestamp_ms = to_time_epoch_ms(&uuid_v7)?;
    println!("Extracted timestamp (ms since epoch): {}", timestamp_ms);

    // Example with a non-v7 UUID (will result in an error)
    let uuid_v4 = uuid_extra::new_v4();
    match to_time_epoch_ms(&uuid_v4) {
        Ok(ts) => println!("Timestamp (this should not happen for v4): {}", ts),
        Err(e) => println!("Error extracting time from v4: {}", e),
    }

    Ok(())
}
```

### Error Handling

Functions that can fail (e.g., decoding) return `uuid_extra::Result<T>`, which is an alias for `core::result::Result<T, uuid_extra::Error>`. The `uuid_extra::Error` enum covers various error scenarios, such as invalid input strings or incorrect UUID versions for certain operations.

## Modules and Structure

The crate is organized into several modules:

-   `extra_uuid`: Core UUID generation (`new_v4`, `new_v7`) and v7 timestamp extraction (`to_time_epoch_ms`).
-   `extra_base58`: Functions for Base58 encoding (`new_v4_b58`, `new_v7_b58`) and decoding (`from_b58`).
-   `extra_base64`: Functions for various Base64 encodings (standard, URL-safe, URL-safe no-pad) and their corresponding decoding functions.
-   `error`: Defines the `Error` enum and `Result<T>` type alias for the crate.
-   `support`: Internal helper functions.

This structure keeps the functionalities related to different encoding schemes separate and organized.
