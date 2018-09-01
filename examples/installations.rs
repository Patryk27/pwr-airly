extern crate pwr_airly;

use pwr_airly::AirlyClient;

fn main() {
    let airly = AirlyClient::new("");

    println!("{:?}", airly.installations().get(100));
}