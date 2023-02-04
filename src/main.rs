mod interfaces;

use interfaces::MARKETS;
use rocket::fs::{FileServer, relative};
use rocket::http::Status;
use rocket::response::stream::{Event as RocketEvent, EventStream};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use unitn_market_2022::market::good_label::GoodLabel;

use crate::interfaces::LogEvent;

#[macro_use]
extern crate rocket;


#[post("/currentGoods/<market>", data = "<goods>")]
fn post_current_goods(goods: Json<Vec<GoodLabel>>, market: &str) -> Status{
    if !MARKETS.contains(&market) {
        return Status::NotFound;
    }
    Status::Accepted
}

//All events performed by the trader are sent to this function
#[post("/log", data="<event>")]
fn post_new_event(event: Json<LogEvent>,queue: &State<Sender<LogEvent>>) {
    println!("{:?}", event);
    let _res = queue.send(event.into_inner());
}

#[get("/log")]
fn get_log(queue: &State<Sender<LogEvent>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield RocketEvent::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<LogEvent>(16536).0)
        .mount("/", routes![post_current_goods, post_new_event, get_log])
        .mount("/", FileServer::from(relative!("frontEnd/build")))
}
