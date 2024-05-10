use crate::Text;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Rect, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::stdout;
use std::time::Duration;

use actix::prelude::*;

pub struct StdoutRenderer {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    text: Text,
}

impl Default for StdoutRenderer {
    fn default() -> Self {
        let terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

        Self {
            terminal,
            text: Text::default(),
        }
    }
}

impl Actor for StdoutRenderer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        self.terminal.clear().unwrap();
        ctx.notify(Tick);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

impl Handler<Text> for StdoutRenderer {
    type Result = ();

    fn handle(&mut self, text: Text, _: &mut Self::Context) {
        self.text = text;
    }
}

#[derive(Default, Message)]
#[rtype(result = "()")]
pub struct Tick;

impl Handler<Tick> for StdoutRenderer {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
        self.terminal
            .draw(|frame| {
                frame.render_widget(
                    Paragraph::new(&self.text.rows[0]).white().on_black(),
                    Rect::new(0, 0, 20, 1),
                );
                frame.render_widget(
                    Paragraph::new(&self.text.rows[1]).white().on_black(),
                    Rect::new(0, 1, 20, 1),
                );
                frame.render_widget(
                    Paragraph::new(&self.text.rows[2]).white().on_black(),
                    Rect::new(0, 2, 20, 1),
                );
                frame.render_widget(
                    Paragraph::new(&self.text.rows[3]).white().on_black(),
                    Rect::new(0, 3, 20, 1),
                );
            })
            .unwrap();

        // for row in &self.text.rows {
        //     println!("{}", row);
        // }

        ctx.notify_later(Tick, Duration::from_secs(5));
    }
}
