extern crate pwr_airly;

use pwr_airly::AirlyClient;

fn main() {
    let airly = AirlyClient::new("my-secret-key");

    /**
     * To query about a specific installation, you can use the `get()` method.
     *
     * It's an equivalent of the <https://developer.airly.eu/docs#endpoints.installations.getbyid>
     * endpoint.
     */
    println!("{:#?}", airly.installations().get(100));

    /**
     * If you don't care about a specific installation, but rather think of 'show me anything
     * nearby ...', you can always fall back to the `get_nearest()` method, which returns a list of
     * installations located nearby given point.
     */
    println!("{:#?}", airly.installations().get_nearest(50.062006, 19.940984));
}
