mod interfaces;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;

use crate::rocket::futures::StreamExt;
use interfaces::{CurrentBuyRate, CurrentGood, CurrentSellRate, TraderGood};
use rand::{rngs::OsRng, Rng};
use reqwest_eventsource::{Event as ReqEvent, EventSource};
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{Event as RocketEvent, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use serde::Serialize;
use tokio::sync::broadcast::Receiver;
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::good_label::GoodLabel;

use crate::interfaces::{Channels, LogEvent, MsgMultiplexed};

#[macro_use]
extern crate rocket;

//Struct used to count the days passed during the simulation
struct Time(AtomicU32);

//Struct used to store in memory the logs, in order to provied them to clients that connect after the simulation started
struct Cache(RwLock<Vec<LogEvent>>);

//Function used to receive from the trader the current GoodLabels held by the market referenced
#[post("/currentGoodLabels/<market>", data = "<goods>")]
fn post_current_good_labels(
    goods: Json<Vec<GoodLabel>>,
    market: &str,
    current_goods: &State<[Sender<CurrentGood>; 3]>,
    current_buy_rate: &State<[Sender<CurrentBuyRate>; 3]>,
    current_sell_rate: &State<[Sender<CurrentSellRate>; 3]>,
    time: &State<Time>,
) {
    let index = match market {
        "BFB" => 0,
        "RCNZ" => 1,
        "ZSE" => 2,
        _ => return,
    };
    //I add all the goodLabels to the channels
    goods.iter().for_each(|good_label| {
        let time = time.0.load(Ordering::Relaxed);
        let kind = good_label.good_kind;

        let c_goods = CurrentGood {
            value: good_label.quantity,
            kind,
            time,
        };
        let _res = current_goods[index].send(c_goods);

        let c_buy_rate = CurrentBuyRate {
            value: good_label.exchange_rate_buy,
            kind,
            time,
        };
        let _res = current_buy_rate[index].send(c_buy_rate);

        let c_sell_rate = CurrentSellRate {
            value: good_label.exchange_rate_sell,
            kind,
            time,
        };
        let _res = current_sell_rate[index].send(c_sell_rate);
    });
}

#[post("/traderGoods", data="<goods>")]
fn post_trader_goods(mut goods: Json<Vec<TraderGood>>, time: &State<Time>, queue: &State<Sender<TraderGood>>) {

    let time = time.0.load(Ordering::Relaxed);
    goods.iter_mut().for_each(|good| {
        good.time = time;
        let _res = queue.send(*good);
    });
}

//Function used to send to the client the current goods held by the market referenced
#[get("/currentTraderGoods")]
fn get_trader_goods(
    rec: &State<Sender<TraderGood>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = rec.subscribe();
    EventStream! {
        loop {
            let msg =
                select! {
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

//Function that provides the eventStream for all the kind of information regarding the market (currentGoods, exchangeBuyRate, exchangeSellRate)
fn send_event_stream<T: Clone + Serialize>(
    mut rx: Receiver<T>,
    index: usize,
    mut end: Shutdown,
) -> EventStream![] {
    EventStream! {
        loop {
            if index == 3 {
                break;
            }
            let msg =
                select! {
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

//Function used to send to the client the current goods held by the market referenced
#[get("/currentGoods/<market>")]
fn get_current_goods(
    market: &str,
    rec: &State<[Sender<CurrentGood>; 3]>,
    end: Shutdown,
) -> EventStream![] {
    let index = match market {
        "BFB" => 0,
        "RCNZ" => 1,
        "ZSE" => 2,
        _ => 3,
    };
    //If the index is 3 the market sent is not valid
    send_event_stream(rec[index % 3].subscribe(), index, end)
}

//Function used to send to the client the current exchange buy rates of the market referenced
#[get("/currentBuyRate/<market>")]
fn get_current_buy_rate(
    market: &str,
    rec: &State<[Sender<CurrentBuyRate>; 3]>,
    end: Shutdown,
) -> EventStream![] {
    let index = match market {
        "BFB" => 0,
        "RCNZ" => 1,
        "ZSE" => 2,
        _ => 3,
    };
    //If the index is 3 the market sent is not valid
    send_event_stream(rec[index % 3].subscribe(), index, end)
}

//Function used to send to the client the current exchange sell rates of the market referenced
#[get("/currentSellRate/<market>")]
fn get_current_sell_rate(
    market: &str,
    rec: &State<[Sender<CurrentSellRate>; 3]>,
    end: Shutdown,
) -> EventStream![] {
    let index = match market {
        "BFB" => 0,
        "RCNZ" => 1,
        "ZSE" => 2,
        _ => 3,
    };
    //If the index is 3 the market sent is not valid
    send_event_stream(rec[index % 3].subscribe(), index, end)
}

#[get("/currentMarket/<market>")]
async fn get_current_market(market: &str, mut end: Shutdown) -> EventStream![] {
    let index = match market {
        "BFB" => 0,
        "RCNZ" => 1,
        "ZSE" => 2,
        _ => 3,
    };
    let mut source_goods =
        EventSource::get("http://localhost:8000/currentGoods/".to_string() + market);
    let mut source_buy_rate =
        EventSource::get("http://localhost:8000/currentBuyRate/".to_string() + market);
    let mut source_sell_rate =
        EventSource::get("http://localhost:8000/currentSellRate/".to_string() + market);

    EventStream! {
        loop {
            if index == 3 {
                break;
            }
            let msg =
                select! {
                    msg = source_goods.next() => match msg {
                        Some(content) => match content {
                            Ok(ReqEvent::Message(message)) => {
                                MsgMultiplexed{channel: Channels::CurrentGoods, log: message.data}
                            },
                            _ => continue,
                        },
                        None => break,
                    },
                    msg = source_buy_rate.next() => match msg {
                        Some(content) => match content {
                            Ok(ReqEvent::Message(message)) => {
                                MsgMultiplexed{channel: Channels::CurrentBuyRate, log: message.data}
                            },
                            _ => continue,
                        },
                        None => break,
                    },
                    msg = source_sell_rate.next() => match msg {
                        Some(content) => match content {
                            Ok(ReqEvent::Message(message)) => {
                                MsgMultiplexed{channel: Channels::CurrentSellRate, log: message.data}
                            },
                            _ => continue,
                        },
                        None => break,
                    },
                    _ = &mut end => break,
                };
            yield RocketEvent::json(&msg);
        }
    }
}

//Function to simulate a client that sends a new Vec of GoodLabels
#[get("/fakeGoodLabels/<market>")]
async fn fake_good_labels(market: &str, time: &State<Time>) {
    let mut rng = OsRng::default();
    let kinds = [GoodKind::EUR, GoodKind::YUAN, GoodKind::YEN, GoodKind::USD];
    let labels: Vec<GoodLabel> = kinds
        .map(|kind| {
            return GoodLabel {
                good_kind: kind,
                quantity: rng.gen_range(1.0..1000.0),
                exchange_rate_buy: rng.gen_range(0.1..10.22),
                exchange_rate_sell: rng.gen_range(0.1..10.0),
            };
        })
        .to_vec();
    let client = reqwest::Client::new();
    let _res = client
        .post("http://localhost:8000/currentGoodLabels/".to_string() + market)
        .json(&labels)
        .send()
        .await;

    time.0.fetch_add(1, Ordering::Acquire);
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
fn get_log<'a>(
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
        .manage(channel::<TraderGood>(16536).0)
        .manage(Cache(RwLock::new(Vec::new())))
        .manage([
            channel::<CurrentGood>(16536).0,
            channel::<CurrentGood>(16536).0,
            channel::<CurrentGood>(16536).0,
        ])
        .manage([
            channel::<CurrentBuyRate>(16536).0,
            channel::<CurrentBuyRate>(16536).0,
            channel::<CurrentBuyRate>(16536).0,
        ])
        .manage([
            channel::<CurrentSellRate>(16536).0,
            channel::<CurrentSellRate>(16536).0,
            channel::<CurrentSellRate>(16536).0,
        ])
        .mount(
            "/",
            routes![
                post_current_good_labels,
                get_current_goods,
                get_current_buy_rate,
                get_current_sell_rate,
                get_current_market,
                post_new_event,
                get_log,
                fake_good_labels,
                post_trader_goods,
                get_trader_goods
            ],
        )
        .mount(
            "/marketController",
            FileServer::from(relative!("frontEnd/build")).rank(2),
        )
        .mount("/", FileServer::from(relative!("frontEnd/build")).rank(1))
}
