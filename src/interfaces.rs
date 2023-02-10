use std::sync::{
    atomic::{AtomicU32, Ordering},
    RwLock,
};

use serde::{Deserialize, Serialize};
use unitn_market_2022::good::good_kind::GoodKind;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomEventKind {
    Bought,
    Sold,
    LockedBuy,
    LockedSell,
    Wait,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomEvent {
    pub kind: CustomEventKind,
    pub good_kind: GoodKind,
    pub quantity: f32,
    pub price: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEvent {
    #[serde(default)]
    pub time: u32,
    pub event: CustomEvent,
    pub market: String,
    pub result: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CurrentGood {
    pub time: u32,
    pub value: f32,
    pub kind: GoodKind,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CurrentBuyRate {
    pub time: u32,
    pub value: f32,
    pub kind: GoodKind,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CurrentSellRate {
    pub time: u32,
    pub value: f32,
    pub kind: GoodKind,
}

#[derive(Serialize)]
pub enum Channels {
    CurrentGoods,
    CurrentBuyRate,
    CurrentSellRate,
}

#[derive(Serialize)]
pub struct MsgMultiplexed {
    pub channel: Channels,
    pub log: String,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TraderGood {
    #[serde(default)]
    pub time: u32,
    pub kind: GoodKind,
    pub quantity: f32,
}

//Struct used to count the days passed during the simulation
pub struct Time(pub AtomicU32);

impl Time {
    fn increment(&self, quantity: u32) -> u32 {
        self.0.fetch_add(quantity, Ordering::Relaxed) + 1
    }

    pub fn increment_one(&self) -> u32 {
        self.increment(1)
    }

    pub fn get(&self) -> u32 {
        self.0.load(Ordering::Relaxed)
    }
}

//Struct used to store in memory the logs, in order to provied them to clients that connect after the simulation started
pub struct CacheLogEvent(pub RwLock<Vec<LogEvent>>);

impl CacheLogEvent {
    pub fn add(&self, value: LogEvent) {
        let mut cache_lock = self.0.write().unwrap();
        (*cache_lock).push(value);
    }

    pub fn clone_vec(&self) -> Vec<LogEvent> {
        let lock = self.0.read().unwrap();
        let copy = (*lock).clone();
        drop(lock);
        copy
    }
}

//Struct used to store in memory the state of the trader each day, in order to provied them to clients that connect after the simulation started
pub struct CacheTraderGood(pub RwLock<Vec<TraderGood>>);

impl CacheTraderGood {
    pub fn add(&self, value: TraderGood) {
        let mut cache_lock = self.0.write().unwrap();
        (*cache_lock).push(value);
    }

    pub fn clone_vec(&self) -> Vec<TraderGood> {
        let lock = self.0.read().unwrap();
        let copy = (*lock).clone();
        drop(lock);
        copy
    }
}
