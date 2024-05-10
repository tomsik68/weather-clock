use actix::prelude::*;
use crossterm::event::{self, KeyCode, KeyEventKind};
use std::time::Duration;
use tokio::sync::mpsc::Sender;

pub struct TerminalShutdownMonitor {
    pub tx: Sender<()>,
}

impl Actor for TerminalShutdownMonitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Tick);
    }
}

#[derive(Default, Message)]
#[rtype(result = "()")]
struct Tick;

impl Handler<Tick> for TerminalShutdownMonitor {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    let tx = self.tx.clone();
                    async move {
                        let _ = tx.send(()).await;
                    }
                    .into_actor(self)
                    .wait(ctx);
                }
            }
        }

        ctx.notify_later(Tick, Duration::from_secs(1));
    }
}

pub struct UnixSignalShutdownMonitor {
    pub tx: Sender<()>,
}

impl Actor for UnixSignalShutdownMonitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Tick);
    }
}

impl Handler<Tick> for UnixSignalShutdownMonitor {
    type Result = ();

    fn handle(&mut self, _: Tick, ctx: &mut Self::Context) {
        let tx = self.tx.clone();
        async move {
            tokio::signal::ctrl_c().await.unwrap();
            let _ = tx.send(()).await;
        }
        .into_actor(self)
        .spawn(ctx);
    }
}
