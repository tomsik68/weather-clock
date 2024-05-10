use crate::LimitedString;
use lcd::Display;
use lcd_pcf8574::{ErrorHandling, Pcf8574};

pub struct Lcd {
    display: Display<Pcf8574>,
}

impl Lcd {
    pub fn new_i2c(bus: u8, addr: u16) -> anyhow::Result<Self> {
        let mut dev = Pcf8574::new(bus, addr)?;
        dev.on_error(ErrorHandling::None);

        let mut display = Display::new(dev);
        display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
        display.display(
            lcd::DisplayMode::DisplayOn,
            lcd::DisplayCursor::CursorOff,
            lcd::DisplayBlink::BlinkOff,
        );

        display.clear();
        display.home();
        Ok(Self { display })
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.display.clear();
        self.display.home();
        Ok(())
    }

    pub fn line_1(&mut self, ls: &LimitedString) -> anyhow::Result<()> {
        self.display.position(0, 0);
        self.display.print(ls.as_str());
        Ok(())
    }

    pub fn line_2(&mut self, ls: &LimitedString) -> anyhow::Result<()> {
        self.display.position(0, 1);
        self.display.print(ls.as_str());
        Ok(())
    }

    pub fn line_3(&mut self, ls: &LimitedString) -> anyhow::Result<()> {
        self.display.position(0, 2);
        self.display.print(ls.as_str());
        Ok(())
    }

    pub fn line_4(&mut self, ls: &LimitedString) -> anyhow::Result<()> {
        self.display.position(0, 3);
        self.display.print(ls.as_str());
        Ok(())
    }
}
