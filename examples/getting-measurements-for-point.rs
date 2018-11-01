extern crate pwr_airly;

use pwr_airly::AirlyClient;
use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<Error>> {
    let airly = AirlyClient::new("my-api-key");

    // To query for measurements near a specific point, you can use the `measurements().for_point()` method.
    // It models the <https://developer.airly.eu/docs#endpoints.measurements.installation> endpoint.
    let response = airly.measurements().for_point(50.062006, 19.940984)?;

    // After the response has been fetched, you can use the `rate_limit()` method to access
    // information about the rate-limiting (i.e. how many requests per API key you can perform), and
    // you can use the `model()` method to access the model (contents) of the response.
    println!("{:#?}", response.rate_limit());
    println!("{:#?}", response.model());

    Ok(())
}
