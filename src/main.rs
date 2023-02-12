mod interfaces;

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::RwLock;

use interfaces::{
    Block, CacheLogEvent, CacheTraderInfo, CurrentBuyRate, CurrentGood, CurrentSellRate, Delay,
    Time, TraderInfo,
};
use rand::{rngs::OsRng, Rng};
use reqwest_eventsource::{Event as ReqEvent, EventSource};
use crate::rocket::futures::StreamExt;
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{Event as RocketEvent, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use serde::Serialize;
use tokio::sync::broadcast::Receiver;
use unitn_market_2022::good::consts::{
    DEFAULT_EUR_USD_EXCHANGE_RATE, DEFAULT_EUR_YEN_EXCHANGE_RATE, DEFAULT_EUR_YUAN_EXCHANGE_RATE,
};
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::market::good_label::GoodLabel;

use crate::interfaces::{Channels, LogEvent, MsgMultiplexed};

#[macro_use]
extern crate rocket;

//Function used to receive from the trader the current GoodLabels held by the market referenced
#[post("/currentGoodLabels/<market>", data = "<goods>")]
fn post_current_good_labels(
    mut goods: Json<Vec<GoodLabel>>,
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
    goods.sort_by(|a, b| a.good_kind.to_string().cmp(&b.good_kind.to_string()));
    goods.iter().for_each(|good_label| {
        let time = time.get();
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

//Function used to receive the goods owned by the trader
#[post("/traderGoods", data = "<goods>")]
fn post_trader_goods(
    mut goods: Json<Vec<TraderInfo>>,
    time: &State<Time>,
    queue: &State<Sender<TraderInfo>>,
    cache: &State<CacheTraderInfo>,
) {
    let time = time.get();

    goods.sort_by(|a, b| a.kind.to_string().cmp(&b.kind.to_string()));
    let mut tot = 0.0;

    goods.iter_mut().for_each(|good| {
        good.time = time;
        tot += good.calc_value();
        let _res = queue.send(*good);
        cache.add(good.clone());
    });
    let total = TraderInfo {
        time,
        kind: interfaces::TraderGraphSeries::TOT,
        quantity: tot,
    };
    let _res = queue.send(total);
    cache.add(total);
}

//Function used to send to the client the current goods held by the market referenced
#[get("/currentTraderGoods")]
fn get_trader_goods<'a>(
    rec: &'a State<Sender<TraderInfo>>,
    mut end: Shutdown,
    cache: &'a State<CacheTraderInfo>,
    stop: &'a State<Block>,
) -> EventStream![RocketEvent + 'a] {
    let mut rx = rec.subscribe();
    //Clone of the current situation in order to provide to the client all the states that have been previously recorded
    let mut cop = cache.iter();
    EventStream! {
        loop {
            if stop.0.load(Ordering::Relaxed) {
                //To shutdown gracefully (if not needed, just continue)
                select! {
                    _ = &mut end => break,
                    _ = async{true} => continue,
                }
            }
            let msg = match cop.next() {
                Some(event) => event,
                None => {
                    select! {
                        msg = rx.recv() => match msg {
                            Ok(msg) => msg,
                            Err(RecvError::Closed) => break,
                            Err(RecvError::Lagged(_)) => continue,
                        },
                        _ = &mut end => break,
                }
            }};
            yield RocketEvent::json(&msg);
        }
    }
}

//Function that provides the current exchange rates (the defaults) to the clients
#[get("/defaultExchange")]
fn get_default_exchange() -> Json<Vec<(GoodKind, f32)>> {
    let arr: Vec<(GoodKind, f32)> = [
        (GoodKind::EUR, 1.0),
        (GoodKind::USD, DEFAULT_EUR_USD_EXCHANGE_RATE),
        (GoodKind::YEN, DEFAULT_EUR_YEN_EXCHANGE_RATE),
        (GoodKind::YUAN, DEFAULT_EUR_YUAN_EXCHANGE_RATE),
    ]
    .to_vec();
    Json(arr)
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
async fn get_current_market<'a>(market: &'a str, mut end: Shutdown) -> EventStream![] {
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
                            }
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

    time.increment_one();
}

//All events performed by the trader are sent to this function
#[post("/log", data = "<event>")]
async fn post_new_event(
    mut event: Json<LogEvent>,
    queue: &State<Sender<LogEvent>>,
    time: &State<Time>,
    cache: &State<CacheLogEvent>,
) {
    //Check if the action had a positive result (no error). If so, increment the day
    if event.result {
        event.time = time.increment_one();
    } else {
        event.time = time.get();
    }
    //Send the event to the receivers
    let content = event.into_inner();
    let _res = queue.send(content.clone());
    cache.add(content)
}

//Function that provides the event received from the trader
#[get("/log")]
fn get_log<'a>(
    queue: &State<Sender<LogEvent>>,
    mut end: Shutdown,
    cache: &State<CacheLogEvent>,
    stop: &'a State<Block>,
) -> EventStream![RocketEvent + 'a] {
    //Subscribe to the queue for future Logs
    let mut rx = queue.subscribe();
    //Clone of the current situation in order to provide to the client all the Logs that have been previously recorded
    let mut cop = cache.iter();
    EventStream! {
        loop {
            if stop.0.load(Ordering::Relaxed) {
                //To shutdown gracefully (if not needed, just continue)
                select! {
                    _ = &mut end => break,
                    _ = async{true} => continue,
                }
            }
            //If there are some old Logs to send
            let msg = match cop.next() {
                Some(event) => event,
                None => {
                    //If there are no old Logs keep listening for next
                    select! {
                        msg = rx.recv() => match msg {
                            Ok(msg) => msg,
                            Err(RecvError::Closed) => break,
                            Err(RecvError::Lagged(_)) => continue,
                        },
                        _ = &mut end => break,
                }
            }};

            yield RocketEvent::json(&msg);
        }
    }
}

#[post("/block")]
fn block(stop: &State<Block>) {
    stop.0.store(true, Ordering::Relaxed)
}

#[post("/unblock")]
fn unblock(stop: &State<Block>) {
    stop.0.store(false, Ordering::Relaxed)
}

#[post("/delay", data = "<data>")]
fn set_delay(data: Json<String>, state_delay: &State<Delay>) {
    let content = data.0.parse::<u32>().unwrap();
    state_delay.set(content);
}

#[get("/delay")]
fn get_delay(state_delay: &State<Delay>) -> Json<u32> {
    Json(state_delay.get())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Time(AtomicU32::new(0)))
        .manage(channel::<LogEvent>(16536).0)
        .manage(CacheLogEvent(RwLock::new(Vec::new())))
        .manage(channel::<TraderInfo>(16536).0)
        .manage(CacheTraderInfo(RwLock::new(Vec::new())))
        .manage(Block(AtomicBool::new(false)))
        .manage(Delay {
            delay: AtomicU32::new(1000),
        })
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
                get_trader_goods,
                get_default_exchange,
                block,
                unblock,
                get_delay,
                set_delay
            ],
        )
        .mount(
            "/marketController",
            FileServer::from(relative!("frontEnd/build")).rank(2),
        )
        .mount("/", FileServer::from(relative!("frontEnd/build")).rank(1))
}
