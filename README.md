# moonlander_server
A server for communicating with my Moonlander keyboard. Now it implements the functionality for per-application language switch between English and Russian layouts.
The server is intended to be used with this [firmware](https://github.com/zamazan4ik/moonlander).

For now the only supported host config is Linux/KDE. However, it should be easy to adopt it ti another OS/DE.

## How to build
* Install [Rust](https://www.rust-lang.org/) (1.59 or newer)
* Clone the repo
* Install corresponding development C headers for Dbus and libusb
* Run `cargo build`

## How to run
You can choose any way, how to run it: simply from command-line, systemd based service, Docker (in this case be careful to set up work with the host Dbus properly).

## Useful links
* Base Python [implementation](https://gist.github.com/Nuigurumi777/47788978b556d1ce258d83f60578a26c)
