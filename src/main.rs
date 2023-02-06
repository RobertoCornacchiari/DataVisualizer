mod interfaces;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;

use interfaces::MARKETS;
use rocket::fs::{relative, FileServer};
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

//Struct used to store in memory the logs, in ordet to provied them to clients that connect after the simulation started
struct Cache(RwLock<Vec<LogEvent>>);

#[post("/currentGoods/<market>", data = "<goods>")]
fn post_current_goods(goods: Json<Vec<GoodLabel>>, market: &str) -> Status {
    if !MARKETS.contains(&market) {
        return Status::NotFound;
    }
    Status::Accepted
}

//All events performed by the trader are sent to this function
#[post("/log", data = "<event>")]
async fn post_new_event(
    mut event: Json<LogEvent>,
    queue: &State<Sender<LogEvent>>,
    time: &State<Time>,
    cache: &State<Cache>,
) {
    //Check if the action had a positive result (no error). If so, increment the day
    if event.result {
        event.time = time.0.fetch_add(1, Ordering::Relaxed) + 1;
    } else {
        event.time = time.0.load(Ordering::Relaxed);
    }
    //Send the event to the receivers
    let content = event.into_inner();
    let _res = queue.send(content.clone());
    let mut cache_lock = cache.0.write().unwrap();
    (*cache_lock).push(content);
}

//Function that provides the event received from the trader
#[get("/log")]
async fn get_log<'a>(
    queue: &State<Sender<LogEvent>>,
    mut end: Shutdown,
    cache: &State<Cache>,
) -> EventStream![] {
    //Subscribe to the queue for future Logs
    let mut rx = queue.subscribe();
    //Clone of the current situation in order to provide to the client all the Logs that have been previously recorded
    let cache_lock = cache.0.read().unwrap();
    let mut cop = (*cache_lock).clone();
    drop(cache_lock);
    EventStream! {
        loop {
            //If there are some old Logs to send
            let msg = if cop.len() > 0 {
                cop.remove(0)
            }
            //If there are no old Logs keep listening for next
            else {
                select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            }};
            
            yield RocketEvent::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Time(AtomicU32::new(0)))
        .manage(channel::<LogEvent>(16536).0)
        .manage(Cache(RwLock::new(Vec::new())))
        .mount("/", routes![post_current_goods, post_new_event, get_log])
        .mount("/", FileServer::from(relative!("frontEnd/build")))
}
