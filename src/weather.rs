use actix::prelude::*;
use std::sync::Arc;
use std::time::Duration;

#[derive(Default, Copy, Clone)]
pub struct RainSituation {
    pub rain: bool,
    pub snow: bool,
    pub thunder: bool,
}

impl<'a, I: Iterator<Item = &'a crate::owm::Weather>> From<I> for RainSituation {
    fn from(mut i: I) -> Self {
        let rain = i.any(|w| w.id / 100 == 5 || w.id / 100 == 2 || w.id / 100 == 3);
        let thunder = i.any(|w| w.id / 100 == 2);
        let snow = i.any(|w| w.id / 100 == 6);
        RainSituation {
            rain,
            thunder,
            snow,
        }
    }
}

#[derive(Clone)]
pub struct WeatherEntry {
    pub temperature: i8,
    pub rain_situation: RainSituation,
    pub label: &'static str,
}

impl Default for WeatherEntry {
    fn default() -> Self {
        Self {
            temperature: -17,
            rain_situation: RainSituation {
                rain: true,
                snow: true,
                thunder: true,
            },
            label: "",
        }
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct WeatherData {
    pub entries: [WeatherEntry; 5],
}

impl From<crate::owm::Root> for WeatherData {
    fn from(owm: crate::owm::Root) -> Self {
        let current = WeatherEntry {
            temperature: owm.current.feels_like.round() as i8,
            rain_situation: RainSituation::from(owm.current.weather.iter()),
            label: "now",
        };

        let hour_1 = &owm.hourly[2];
        let hour_1 = WeatherEntry {
            temperature: hour_1.feels_like.round() as i8,
            rain_situation: RainSituation::from(hour_1.weather.iter()),
            label: "+2h",
        };
        let hour_2 = &owm.hourly[4];
        let hour_2 = WeatherEntry {
            temperature: hour_2.feels_like.round() as i8,
            rain_situation: RainSituation::from(hour_2.weather.iter()),
            label: "+4h",
        };
        let hour_3 = &owm.hourly[6];
        let hour_3 = WeatherEntry {
            temperature: hour_3.feels_like.round() as i8,
            rain_situation: RainSituation::from(hour_3.weather.iter()),
            label: "+6h",
        };

        // daily[0] is today
        let tomorrow = &owm.daily[1];
        let tomorrow = WeatherEntry {
            temperature: tomorrow.feels_like.day.round() as i8,
            rain_situation: RainSituation::from(tomorrow.weather.iter()),
            label: "tmr",
        };

        Self {
            entries: [current.clone(), hour_1, hour_2, hour_3, tomorrow],
        }
    }
}

impl WeatherData {
    pub fn zero() -> Self {
        Self {
            entries: Default::default(),
        }
    }
}

pub struct Weather {
    subscriber: Recipient<WeatherData>,
    owm_url: Arc<String>,
}

#[derive(Default, Message)]
#[rtype(result = "()")]
struct Tick;

impl Weather {
    pub fn with_subscriber(
        subscriber: Recipient<WeatherData>,
        latitude: String,
        longitude: String,
        units: String,
        appid: String,
    ) -> Self {
        let owm_url = format!("https://api.openweathermap.org/data/3.0/onecall?units={units}&lat={latitude}&lon={longitude}&appid={appid}");
        Self {
            subscriber,
            owm_url: Arc::new(owm_url),
        }
    }
}

impl Actor for Weather {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Tick);
    }
}

impl Handler<Tick> for Weather {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
        let sub = self.subscriber.clone();
        let ticker = ctx.address();
        let owm_url = Arc::clone(&self.owm_url);

        async move {
            let sub = sub;

            let w = crate::owm::fetch_current_weather(&owm_url).await;
            match w {
                Ok(w) => {
                    let _ = sub.send(w.into()).await;
                    tokio::time::sleep(Duration::from_secs(3600)).await;
                    ticker.send(Tick).await.unwrap();
                }
                Err(e) => {
                    dbg!(e);
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    ticker.send(Tick).await.unwrap();
                }
            };
        }
        .into_actor(self)
        .spawn(ctx);
    }
}
