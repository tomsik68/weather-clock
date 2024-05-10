This is a Rust program that controls a DIY weather clock powered by Raspberry PI and PCF8574 controller.

The PI uses its I2C bus to talk to the PCF8574 controller which is attached to the LCD display.

# Resources

- [Testing the LCD display](https://www.circuitbasics.com/raspberry-pi-i2c-lcd-set-up-and-programming/)
- [OpenWeatherMap](https://openweathermap.org/)

# Development dependencies

The following dependencies will enable you to cross-compile the binary for Raspberry PI:

- [Rust](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- cross `cargo install cross`
- [Docker](https://docs.docker.com/get-docker/)

# Installation

This instruction manual comes with zero warranty. If you don't agree, please don't proceed.
It is strongly recommended to read [Testing the LCD display](https://www.circuitbasics.com/raspberry-pi-i2c-lcd-set-up-and-programming/) before proceeding. This installation manual also isn't very detailed. Feel free to ask questions or contribute if you understand how to make this work.

1. Run `cross build --release --target armv7-unknown-linux-musleabihf -v`
1. Copy `target/armv7-unknown-linux-musleabihf/release/weather-clock` to `/usr/local/bin/weather-clock` on the Raspberry PI
1. Install raspbian on the Raspberry PI
1. Connect the PCF8574 controller to the Raspberry PI via I2C bus
1. Run a python script from the site above to make sure your display works properly. Protip: make sure to adjust contrast if you can't see anything on the display.
1. Create a configuration file based on `weather-clock.example.toml`
1. Copy your configuration file to `/etc/weather-clock.toml` on the Raspberry PI
1. Copy `weather-clock.service` to /etc/systemd/system/weather-clock.service` on the Raspberry PI
1. Run `sudo systemctl daemon-reload` on Rasberry PI
1. Run `sudo systemctl enable --now weather-clock` on Rasberry PI

After all this, the weather clock should start automatically each time your Raspberry PI starts.
