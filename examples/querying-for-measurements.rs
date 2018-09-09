extern crate pwr_airly;

use pwr_airly::AirlyClient;

fn main() {
    let airly = AirlyClient::new("my-secret-key");

    println!("{:#?}", airly.measurements().get(100));
}
