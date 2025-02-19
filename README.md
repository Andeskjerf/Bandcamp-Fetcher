# what is this?!

This is an application that automatically downloads and unzips your Bandcamp purchases to a specified directory.

# how do I use this?

You can run it once to get any missing purchases, or set it on a systemd timer or crontab to schedule it.

## args
* arg1: your Bandcamp username
* arg2: identity cookie (get this from your logged in browser)
* arg3: path to save any downloads

# how does it work?

It'll check the directory you specified earlier to check if a album / single / etc should be downloaded or not. It creates and expects a folder structure like this:

```
top dir
|
-> artist / band
|  |
|  -> album / single / ep / etc
-> artist / band
   |
   -> album / single / ep / etc
```

So if there's no directory with the same name as the album you're trying to download exists, it'll create that directory and download.

# how can I schedule this?

You could add it to crontab, or use systemd units and timers. See below for examples

`bandcamp-fetcher.service`
```systemd
[Unit]
Description="Tool to automatically get your Bandcamp purchases"

[Service]
Type=oneshot
User=user
Group=group
ExecStart="bandcamp-fetcher <username> <identity> <path>"
```

This will run every 10 minutes
`bandcamp-fetcher.timer`
```systemd.timer
[Unit]
PartOf=bandcamp-fetcher.service

[Timer]
OnBootSec=10m
OnUnitActiveSec=10m
Unit=bandcamp-fetcher.service


[Install]
WantedBy=timers.target
```
