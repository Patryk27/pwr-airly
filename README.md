pwr_airly
---------

[![Version](https://img.shields.io/crates/v/pwr_airly.svg)](https://crates.io/crates/pwr_airly)
[Documentation](https://docs.rs/pwr_airly)

`pwr_airly` is an unofficial client for the [Airly's v2 API](https://developer.airly.eu/docs).

# Examples

A few examples are ready for you inside the `examples` directory - just replace the `my-api-key` string with your
actual key and you'll be able to run them with `cargo run --example example-name`.

As for a sneak peek, here's the `getting-measurements-for-installation.rs`:

```rust
use std::error::Error;
use std::result::Result;

use pwr_airly::AirlyClient;

fn main() -> Result<(), Box<Error>> {
    let airly = AirlyClient::new("my-api-key");

    // To query for measurements of a specific installation, you can use the `measurements().get()` method.
    // It models the <https://developer.airly.eu/docs#endpoints.measurements.installation> endpoint.
    let response = airly.measurements().get(250)?;

    // After the response has been fetched, you can use the `rate_limit()` method to access
    // information about the rate-limiting (i.e. how many requests per API key you can perform), and
    // you can use the `model()` method to access the model (contents) of the response.
    println!("{:#?}", response.rate_limit());
    println!("{:#?}", response.model());

    Ok(())
}
```

# Roadmap

1. Implement all the `/v2/meta` endpoints.
2. Add support for asynchronous requests.

# License

```
Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```
