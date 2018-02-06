# What is this project?

[Airly](https://airly.eu/pl/) has a pretty nice weather sensors API which I'm using in my own project - I thought it'd 
be nice to share my code in case someone wants to, say, use it in their own application. 

tl;dr: this library is a simple, unofficial Rust client for [Airly API](https://developer.airly.eu/docs). 

# Show me some code example.

Before you can access the API, you must obtain your personal API key (which you can get by creating a free account at
https://developer.airly.eu).

```rust
extern crate pwr_airly;

use pwr_airly::AirlyClient;

fn main() {
    let airly = AirlyClient::new("your API key");
    let sensor = airly.get_nearest_sensor(50.0, 19.0).unwrap();
    
    println!("{:#?}", sensor);
}
```

# What are all the supported functions?

Airly Endpoint                       | AirlyClient method
------------------------------------ | ------------------
`GET /v1/sensors`                    | `get_sensor(sensor_id: u32) -> Sensor`
`GET /v1/nearestSensor/measurements` | `get_nearest_sensor(latitude: f32, longitude: f32) -> Sensor`
`GET /v1/sensor/measurements`        | `get_sensor_measurements(sensor_id: u32) -> Measurements`
`GET /v1/mapPoint/measurements`      | `get_map_point_measurements(latitude: f32, longitude: f32) -> Measurements`

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