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

mod api;


fn main() {
    println!("one day, something will be happening here");
}

