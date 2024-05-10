use crate::Text;
use actix::prelude::*;
use std::time::Duration;

pub struct LcdRenderer {
    // 1
    bus: u8,
    // 0x27
    addr: u16,
    text: Text,
}

impl LcdRenderer {
    pub fn using_bus_and_addr(bus: u8, addr: u16) -> Self {
        Self {
            bus,
            addr,
            text: Text::default(),
        }
    }
}

#[derive(Default, Message)]
#[rtype(result = "()")]
pub struct Tick;

impl Actor for LcdRenderer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Tick);
    }
}

impl Handler<Tick> for LcdRenderer {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
        // explicitly discard the result of LCD write. If this update failed, the next one will
        // likely succeed
        let _ = (|| -> Result<_, anyhow::Error> {
            let mut lcd = crate::lcd::Lcd::new_i2c(self.bus, self.addr)?;
            lcd.clear()?;
            lcd.line_1(&self.text.rows[0])?;
            lcd.line_2(&self.text.rows[1])?;
            lcd.line_3(&self.text.rows[2])?;
            lcd.line_4(&self.text.rows[3])?;
            Ok(())
        })();

        ctx.notify_later(Tick, Duration::from_secs(30));
    }
}

impl Handler<Text> for LcdRenderer {
    type Result = ();

    fn handle(&mut self, text: Text, _: &mut Self::Context) {
        self.text = text;
    }
}
