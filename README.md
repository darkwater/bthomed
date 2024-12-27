bthomed
=======

Background service that listens for [BTHome](https://bthome.io/) packets and
exposes them over HTTP on `/metrics`. Super minimal at the moment. Source is
small enough to read and understand.

Installation
------------

```bash
$ cargo install --git=https://github.com/darkwater/bthomed
```

Usage
-----

```bash
$ bthomed
```

There's also a systemd service file you can put on your system.

Example output
--------------

```
bthome_humidity{device="ATC_2262AE"} 62.35
bthome_battery{device="ATC_2262AE"} 27
bthome_voltage{device="ATC_2262AE"} 2.3930001
bthome_temperature{device="ATC_2262AE"} 5.39
bthome_rssi{device="ATC_2262AE"} -75
bthome_power{device="ATC_2262AE"} 1
```
