// working curl to fetch albums, using our assigned cookies
// will return a download URL we can use to fetch a zip containing the flac for said album
//
// identity appears to be the most important / only cookie we need?
// ----
// get download URL
// ---- PARAMS
// * id= // NEEDED!! this refers to the album i want to download. is in the XHR response
// * .rand= // no idea what this does, but does not appear to be needed
// * sitem_id= // NEEDED!! appears to be important, API returns error if not included. is in the DOM
// * sig= // NEEDED!! the value can be found in the DOM for a logged in users collection page
// curl \
//   --cookie "identity=redacted;" \
//   -H "Origin: https://bandcamp.com" \
//   -H "Accept: application/json, text/javascript, */*; q=0.01" \
//   "https://popplers5.bandcamp.com/statdownload/album?enc=flac&id=1405377090&sig=8d51978c9a34ac46a293adfa490bc192&sitem_id=314614390&.vrs=1"
// ----
// get collection summary
// curl \
//   --cookie "identity=redacted;" \
//   -H "Origin: https://bandcamp.com" \
//   -H "Accept: application/json, text/javascript, */*; q=0.01" \
//   "https://bandcamp.com/api/fan/2/collection_summary"
// ----
// logging in, not done. requires recaptcha shit. gives us our identity cookie
// curl -v \
//   -X POST \
//   -F user.name="redacted" \
//   -F login.password="redacted" \
//   -F login.twofactor="" \
//   -F login.twofactor_remember="" \
//   -F login.from="fan_page" \
//   -H "Content-Type: application/x-www-form-urlencoded" \
//   -H "Origin: https://bandcamp.com" \
//   https://bandcamp.com/login_cb

use api::bandcamp_api::BandcampAPI;
use std::env;

mod api;

#[tokio::main]
async fn main() -> Result<(), ()> {
    if env::args().len() < 3 {
        println!("error: not enough arguments.");
        println!("args:\n\t1: identity_cookie\n\t2: download_path");
        return Err(());
    }

    let mut identity: String = String::new();
    let mut download_path: String = String::new();
    env::args().enumerate().for_each(|(i, arg)| {
        match i {
            1 => identity = arg,
            2 => download_path = arg,
            // ...how do i ignore this altogether?
            _ => print!(""),
        }
    });

    let api = BandcampAPI::new(&identity);
    let response = api
        .get_collection_summary()
        .await
        .expect("failed to get collection summary");
    println!("{}", response.text().await.unwrap());

    Ok(())
}
