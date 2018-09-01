pwr_airly
---------

[![Version](https://img.shields.io/crates/v/pwr_airly.svg)](https://crates.io/crates/pwr_airly)

[Documentation](https://docs.rs/pwr_airly)

[Airly](https://airly.eu/pl/) has made a pretty nice weather-sensors API, which I'm using in my own project - I thought
I'd give a shot and try to create a client someone else would also consider useful.  

tl;dr This library is a simple, unofficial Rust client for the [Airly API](https://developer.airly.eu/docs). 

# Care to show an example?

**Note:** Before you can access the API, you must obtain your personal API key (which you can get by creating a free
account at <https://developer.airly.eu>).

```rust
extern crate pwr_airly;

use pwr_airly::AirlyClient;

fn main() {
    let airly = AirlyClient::new("Your API key");
    let sensor = airly.get_nearest_sensor(50.0, 19.0).unwrap();
    
    println!("{:#?}", sensor);
}
```

# What are all the supported functions?

Airly endpoint                       | AirlyClient method signature
------------------------------------ | ----------------------------
`GET /v1/sensors`                    | `get_sensor(sensor_id: u32) -> Result<Sensor>`
`GET /v1/nearestSensor/measurements` | `get_nearest_sensor(latitude: f32, longitude: f32) -> Result<Sensor>`
`GET /v1/sensor/measurements`        | `get_sensor_measurements(sensor_id: u32) -> Result<Measurements>`
`GET /v1/mapPoint/measurements`      | `get_map_point_measurements(latitude: f32, longitude: f32) -> Result<Measurements>`

An insightful reader may notice that my nomenclature slightly differs from the original (Airly's) one - this has been 
done deliberately, because Airly's endpoints have been named somewhat misleadingly:
```
GET /v1/sensors/{id} - returns *data* of given sensor,
GET /v1/nearestSensor/measurements - returns *data* of sensor nearest to given location,

GET /v1/sensor/measurements - returns *measurements* of given sensor,
GET /v1/mapPoint/measurements - returns *measurements* of sensor nearest to given location.
``` 

# License

```
Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```