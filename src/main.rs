#![macro_use]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use {
    ector::*,
    embassy_time::{Duration, Timer},
    embassy_futures::join::join,
};

async fn test(addr: DynamicAddress<Request<&'static str, &'static str>>) {
    let _ = embassy_time::with_timeout(Duration::from_micros(1), addr.request("Hello")).await;
}

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    // Example of request response
    static SERVER: ActorContext<Server> = ActorContext::new();

    let address = SERVER.dyn_address();
    let server = SERVER.mount(Server);
    let test = test(address);
    join(server, test).await;
}

pub struct Server;

impl Actor for Server {
    type Message = Request<&'static str, &'static str>;

    async fn on_mount<M>(
        &mut self,
        _: DynamicAddress<Request<&'static str, &'static str>>,
        mut inbox: M,
    ) -> !
    where
        M: Inbox<Self::Message>,
    {
        println!("Server started!");

        loop {
            let motd = inbox.next().await;
            Timer::after(Duration::from_secs(1)).await;
            let m = motd.as_ref().clone();
            motd.reply(m).await;
        }
    }
}
