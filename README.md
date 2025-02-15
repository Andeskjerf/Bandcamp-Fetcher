# what is this?!

This is an application that automatically downloads and unzips your Bandcamp purchases to a specified directory.

It will check the specified directory first to figure out if something is missing or not.

# how do I use this?

You can run it once to get any missing purchases, or set it on a systemd timer or crontab to schedule it.

## args
* arg1: identity cookie (get this from your logged in browser)
* arg2: path to save any downloads
