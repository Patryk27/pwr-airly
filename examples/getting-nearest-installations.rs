use std::error::Error;
use std::result::Result;

use pwr_airly::AirlyClient;

fn main() -> Result<(), Box<Error>> {
    let airly = AirlyClient::new("my-api-key");

    // To query for all installations nearest given point, you can use the `installations().get_nearest()` method.
    // It models the <https://developer.airly.eu/docs#endpoints.installations.nearest> endpoint.
    let response = airly.installations().get_nearest(50.062006, 19.940984)?;

    // After the response has been fetched, you can use the `rate_limit()` method to access
    // information about the rate-limiting (i.e. how many requests per API key you can perform), and
    // you can use the `model()` method to access the model (contents) of the response.
    println!("{:#?}", response.rate_limit());
    println!("{:#?}", response.model());

    Ok(())
}
