# fit-connect-rs

## Description 

Right now this is a POC of an application that will eventually pull data from various fitness sources, export it or sync it to other fitness tools.
Currently, all it does is uses the [withings-rs](https://github.com/qgriffith/withings-rs) rust crate to pull the last weight and sync
it to strava using the [strava-rs](https://github.com/qgriffith/strava-client-rs) crate. It can also pull data out of
strava for the athlete. It currently diplays the following in JSON.

* Athlete Profile
* Athlete Stats for all activities

### Example Use

```shell
fit-connect-rs -h 

fit syncing tool

Usage: fit-connect-rs [OPTIONS] [COMMAND]

Commands:
  withings  
  strava    
  help      Print this message or the help of the given subcommand(s)

Options:
  -l, --log      Optional to enable logging
  -h, --help     Print help
  -V, --version  Print version
```

```shell
fit-connect-rs strava -h
Usage: fit-connect-rs strava [OPTIONS]

Options:
  -r, --register <Only needs ran the first time to register this to your account>
  -a, --get-athlete  
  -s, --get-stats    
  -h, --help         Print help

```

> Note in order to use this you will need to setup the Withings development kit if you plan on using the Withings
> module. [withings](https://github.com/qgriffith/withings-rs?tab=readme-ov-file#use)
> Prior to using this you must create a strava application using your Strava
> Account [strava app](https://developers.strava.com/docs/getting-started/#account)
> You will be prompted to allow the application
> access [strava oauth](https://developers.strava.com/docs/authentication/)
> Then some ENV vars will need to be set which are created when you setup a new application in the link above:
> export STRAVA_CLIENT_ID=72920
> export STRAVA_CLIENT_SECRET=xxxx
> export STRAVA_CONFIG_FILE=/home/xxx/.strava-rs/config.json

## Versions

* [Release Notes](https://github.com/qgriffith/fit-connect-rs/releases)