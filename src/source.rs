use crate::weather::{WeatherData, WeatherEntry};
use crate::Text;
use actix::prelude::*;
use std::time::Duration;
use time::format_description::FormatItem;
use time::macros::offset;

pub struct Source {
    subscriber: Recipient<Text>,
    weather_data: WeatherData,
}

impl Source {
    pub fn with_subscriber(subscriber: Recipient<Text>) -> Self {
        Self {
            subscriber,
            weather_data: WeatherData::zero(),
        }
    }
}

impl Actor for Source {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Tick);
    }
}

fn space_or(value: bool, c: char) -> char {
    match value {
        true => c,
        false => ' ',
    }
}

fn display_rain_situation(we: &WeatherEntry) -> String {
    let rs = we.rain_situation;
    format!(
        "{}{}{} ",
        space_or(rs.rain, 'R'),
        space_or(rs.snow, 'S'),
        space_or(rs.thunder, 'T'),
    )
}

#[derive(Default, Message)]
#[rtype(result = "()")]
struct Tick;

lazy_static::lazy_static! {
    static ref FORMAT: &'static [FormatItem<'static>] = time::macros::format_description!(
        "[weekday repr:short] [month repr:short] [day] [hour]:[minute]:[second]"
    );
}

impl Handler<Tick> for Source {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
        let time = time::OffsetDateTime::now_utc().to_offset(offset!(+2));
        let datetime = time.format(&FORMAT).unwrap().parse().unwrap();

        let temperature = format!(
            "{:3} {:3} {:3} {:3} {:3}",
            self.weather_data.entries[0].temperature,
            self.weather_data.entries[1].temperature,
            self.weather_data.entries[2].temperature,
            self.weather_data.entries[3].temperature,
            self.weather_data.entries[4].temperature,
        )
        .parse()
        .unwrap();

        let conditions = format!(
            "{}{}{}{}{}",
            display_rain_situation(&self.weather_data.entries[0]),
            display_rain_situation(&self.weather_data.entries[1]),
            display_rain_situation(&self.weather_data.entries[2]),
            display_rain_situation(&self.weather_data.entries[3]),
            display_rain_situation(&self.weather_data.entries[4]),
        )
        .parse()
        .unwrap();

        let labels = format!(
            "{:3} {:3} {:3} {:3} {:3}",
            self.weather_data.entries[0].label,
            self.weather_data.entries[1].label,
            self.weather_data.entries[2].label,
            self.weather_data.entries[3].label,
            self.weather_data.entries[4].label,
        )
        .parse()
        .unwrap();

        let text = Text {
            rows: [datetime, labels, temperature, conditions],
        };

        let sub = self.subscriber.clone();
        async move {
            let _ = sub.send(text).await;
        }
        .into_actor(self)
        .wait(ctx);

        ctx.notify_later(Tick, Duration::from_secs(10));
    }
}

impl Handler<WeatherData> for Source {
    type Result = ();

    fn handle(&mut self, wd: WeatherData, _: &mut Self::Context) {
        self.weather_data = wd;
    }
}
