# kraken_api
This is a library that provides access to the kraken.com APIs.

[![docs.rs](https://docs.rs/kraken_api/badge.svg)](https://docs.rs/kraken_api)

# Usage
```rust
use kraken_api::api::Kraken;

fn main() {
    // code to get key, secret and totp goes here
    // ......
    // ......
    
    let kraken = Kraken::new(key, secret, totp);

    kraken.start();
}
```
