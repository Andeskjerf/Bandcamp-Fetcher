# what is this?!

This is an application that automatically downloads and unzips your Bandcamp purchases to a specified directory.

It will check the specified directory first to figure out if something is missing or not.

# how do I use this?

You can run it once to get any missing purchases, or set it on a systemd timer or crontab to schedule it.

## args
* arg1: your Bandcamp username
* arg2: identity cookie (get this from your logged in browser)
* arg3: path to save any downloads

# various ramblings about using Bandcamps API & scraping

working curl to fetch albums, using our assigned cookies
will return a download URL we can use to fetch a zip containing the flac for said album

identity appears to be the most important / only cookie we need?

get download URL

### PARAMS
* id= // NEEDED!! this refers to the album i want to download. is in the XHR response
* .rand= // no idea what this does, but does not appear to be needed
* sitem_id= // NEEDED!! appears to be important, API returns error if not included. is in the DOM
* sig= // NEEDED!! the value can be found in the DOM for a logged in users collection page

```bash
curl \
  --cookie "identity=redacted;" \
  -H "Origin: https://bandcamp.com" \
  -H "Accept: application/json, text/javascript, */*; q=0.01" \
  "https://popplers5.bandcamp.com/statdownload/album?enc=flac&id=1405377090&sig=8d51978c9a34ac46a293adfa490bc192&sitem_id=314614390&.vrs=1"
```

get collection summary. does not actually have much important for us, except for details about each item, like its ID
```bash
curl \
  --cookie "identity=redacted;" \
  -H "Origin: https://bandcamp.com" \
  -H "Accept: application/json, text/javascript, */*; q=0.01" \
  "https://bandcamp.com/api/fan/2/collection_summary"
```

get collection summary HTML DOM, which contains important params to get our download link
```bash
curl \
  --cookie "identity=redacted;" \
  -H "Origin: https://bandcamp.com" \
  -H "Accept: application/json, text/javascript, */*; q=0.01" \
  "https://bandcamp.com/user"
```

logging in, not done. requires recaptcha shit. gives us our identity cookie
```bash
curl -v \
  -X POST \
  -F user.name="redacted" \
  -F login.password="redacted" \
  -F login.twofactor="" \
  -F login.twofactor_remember="" \
  -F login.from="fan_page" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -H "Origin: https://bandcamp.com" \
  https://bandcamp.com/login_cb
```

## getting the download URL in the first place...

in order to actually get the download URL, which we need for the first curl above, we need some special signature URL parameter (no, not the one we've already gotten, but a new one...)... this param is embedded in the DOM for the download page, the page where you get a dropdown to choose the format of your download.

so, we could just do a GET request to get the DOM, find the <select> element and get our signature! easy peasy, right?

fun fact, it appears you don't even need a valid identity cookie to view & download from this page?

### params
- payment_id: we get this from the collections page DOM, using the curl above
- sig: we get this from the collections page DOM, using the curl above
- sitem_id: we get this from the collections page DOM, using the curl above

```bash
curl \
  -H "Origin: https://bandcamp.com" \
  -H "Accept: application/json, text/javascript, */*; q=0.01" \
  "https://bandcamp.com/download?from=collection&payment_id=<payment_id>&sig=<sig>&sitem_id=<sitem_id>"
```

not so fast! there's some js that runs on page load, which then gets all the params required to get a valid URL. bummer!! 

so, what do we do?

- we could use something like selenium (gross)
- figure out what the js is doing, and then replicate the calls it's doing to get the params (yay)
