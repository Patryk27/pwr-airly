# What is this project?

[Airly](https://airly.eu/pl/) has a pretty nice weather sensors API which I'm using in my own project - I thought it'd 
be nice to share my code in case someone wants to, say, use it in their own application. 

tl;dr: this library is a simple Rust client for [Airly API](https://developer.airly.eu/docs). 

# Usage examples

Before you can access the API, you must obtain your personal API key (which you can get by creating a free account at
https://developer.airly.eu).

```rust
extern crate pwr_airly;

use pwr_airly::AirlyClient;

fn main() {
    let airly = AirlyClient::new("your API key");
    let sensor = airly.get_nearest_sensor(50.0, 19.0);
    
    println!("{:#?}", sensor);
}
```

Library is under constant development - expect breaking changes.