mod lcd;
mod lcd_renderer;
mod owm;
mod shutdown_monitor;
mod source;
mod stdout_renderer;
mod weather;

#[derive(Default, Clone, Debug)]
pub struct LimitedString(String);

impl LimitedString {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

use actix::prelude::*;
use std::str::FromStr;

impl FromStr for LimitedString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 20 {
            return Err(());
        }

        Ok(Self(s.to_string()))
    }
}

impl std::fmt::Display for LimitedString {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

impl<'a> Into<ratatui::text::Text<'a>> for &'a LimitedString {
    fn into(self) -> ratatui::text::Text<'a> {
        self.0.as_str().into()
    }
}

#[derive(Default, Message)]
#[rtype(result = "()")]
pub struct Text {
    rows: [LimitedString; 4],
}

/// I2C LCD help: https://www.circuitbasics.com/raspberry-pi-i2c-lcd-set-up-and-programming/
/// OpenWeatherMap help: https://openweathermap.org
///
/// Run the weather clock
#[derive(Debug)]
struct ClockOperator {
    /// Geographical latitude of the place to show weather data for
    latitude: String,

    /// Geographical longitude of the place to show weather data for
    longitude: String,

    /// OpenWeatherMap API key.
    /// See https://home.openweathermap.org/api_keys for more information.
    appid: String,

    /// Units can be: standard, metric or imperial.
    units: String,

    run_mode: RunMode,
}

impl From<Config> for ClockOperator {
    fn from(c: Config) -> Self {
        Self {
            latitude: c.latitude,
            longitude: c.longitude,
            appid: c.appid,
            units: c.units,
            run_mode: RunMode::Lcd {
                bus: c.bus,
                addr: c.addr,
            },
        }
    }
}

#[derive(Clone, Debug)]
enum RunMode {
    /// Run using a real LCD screen
    Lcd {
        /// I2C bus where the display is connected
        bus: u8,

        /// I2C address where the display is connected
        addr: u16,
    },

    /// Run via terminal
    #[allow(unused)]
    Terminal,
}

impl ClockOperator {
    async fn run_terminal(self) {
        let renderer = crate::stdout_renderer::StdoutRenderer::default().start();
        let source = crate::source::Source::with_subscriber(renderer.recipient()).start();
        let _weather = crate::weather::Weather::with_subscriber(
            source.recipient(),
            self.latitude,
            self.longitude,
            self.units,
            self.appid,
        )
        .start();

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let _shutdown_monitor = crate::shutdown_monitor::TerminalShutdownMonitor { tx }.start();

        rx.recv().await;
    }

    async fn run_lcd(self, bus: u8, addr: u16) {
        let renderer = crate::lcd_renderer::LcdRenderer::using_bus_and_addr(bus, addr).start();
        let source = crate::source::Source::with_subscriber(renderer.recipient()).start();
        let _weather = crate::weather::Weather::with_subscriber(
            source.recipient(),
            self.latitude,
            self.longitude,
            self.units,
            self.appid,
        )
        .start();

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let _shutdown_monitor = crate::shutdown_monitor::UnixSignalShutdownMonitor { tx }.start();

        rx.recv().await;
    }
}

#[macro_use]
extern crate configure_me;
configure_me::include_config!();

#[actix_rt::main]
async fn main() {
    let (config, _) =
        Config::including_optional_config_files(&["/etc/weather-clock.toml"]).unwrap_or_exit();
    let co = ClockOperator::from(config);
    match co.run_mode {
        RunMode::Terminal => co.run_terminal().await,
        RunMode::Lcd { bus, addr } => co.run_lcd(bus, addr).await,
    };

    System::current().stop();
}
