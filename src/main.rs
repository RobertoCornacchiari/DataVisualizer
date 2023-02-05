mod interfaces;

use std::sync::atomic::{AtomicU32, Ordering};

use interfaces::MARKETS;
use rocket::fs::{FileServer, relative};
use rocket::http::Status;
use rocket::response::stream::{Event as RocketEvent, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use unitn_market_2022::market::good_label::GoodLabel;

use crate::interfaces::LogEvent;

#[macro_use]
extern crate rocket;

//Struct used to count the days passed during the simulation
struct Time(AtomicU32);

#[post("/currentGoods/<market>", data = "<goods>")]
fn post_current_goods(goods: Json<Vec<GoodLabel>>, market: &str) -> Status{
    if !MARKETS.contains(&market) {
        return Status::NotFound;
    }
    Status::Accepted
}

//All events performed by the trader are sent to this function
#[post("/log", data="<event>")]
fn post_new_event(mut event: Json<LogEvent>,queue: &State<Sender<LogEvent>>, time: &State<Time>) {
    //Check if the action had a positive result (no error). If so, increment the day
    if event.result {
        event.time = time.0.fetch_add(1, Ordering::Relaxed) + 1;
    } else {
        event.time = time.0.load(Ordering::Relaxed);
    }
    //Send the event to the receivers
    let _res = queue.send(event.into_inner());
}

//Function that provides the event received from the trader
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
        .manage(Time(AtomicU32::new(0)))
        .manage(channel::<LogEvent>(16536).0)
        .mount("/", routes![post_current_goods, post_new_event, get_log])
        .mount("/", FileServer::from(relative!("frontEnd/build")))
}
