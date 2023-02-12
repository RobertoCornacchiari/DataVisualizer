use std::{sync::{
    atomic::{AtomicU32, Ordering},
    RwLock,
}, fmt::{Display, Formatter}};

use serde::{Deserialize, Serialize};
use unitn_market_2022::good::{good_kind::GoodKind, consts::{DEFAULT_EUR_USD_EXCHANGE_RATE, DEFAULT_EUR_YEN_EXCHANGE_RATE, DEFAULT_EUR_YUAN_EXCHANGE_RATE}};

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
pub enum TraderGraphSeries {
    EUR, USD, YEN, YUAN, TOT
}

impl Display for TraderGraphSeries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TraderGraphSeries::EUR => { write!(f, "EUR") }
            TraderGraphSeries::YEN => { write!(f, "YEN") }
            TraderGraphSeries::USD => { write!(f, "USD") }
            TraderGraphSeries::YUAN => { write!(f, "YUAN") }
            TraderGraphSeries::TOT => { write!(f, "TOT") }
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TraderInfo {
    #[serde(default)]
    pub time: u32,
    pub kind: TraderGraphSeries,
    pub quantity: f32,
}

impl TraderInfo {
    pub fn calc_value(&self) -> f32 {
        self.quantity / match self.kind {
            TraderGraphSeries::EUR => 1.0,
            TraderGraphSeries::USD => DEFAULT_EUR_USD_EXCHANGE_RATE,
            TraderGraphSeries::YEN => DEFAULT_EUR_YEN_EXCHANGE_RATE,
            TraderGraphSeries::YUAN => DEFAULT_EUR_YUAN_EXCHANGE_RATE,
            TraderGraphSeries::TOT => 1.0,
        }
    }
}

//Struct used to count the days passed during the simulation
pub struct Time(pub AtomicU32);

impl Time {
    fn increment(&self, quantity: u32) -> u32 {
        self.0.fetch_add(quantity, Ordering::Relaxed) + quantity
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

    pub fn iter(&self) -> IterCacheLogEvent {
        IterCacheLogEvent {
            cache: self.clone_vec()
        }
    }
}

pub struct IterCacheLogEvent {
    cache: Vec<LogEvent>
}

impl Iterator for IterCacheLogEvent {
    type Item = LogEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.len() > 0 {
            Some(self.cache.remove(0))
        } else {
            None
        }
    }
}

//Struct used to store in memory the state of the trader each day, in order to provied them to clients that connect after the simulation started
pub struct CacheTraderInfo(pub RwLock<Vec<TraderInfo>>);

impl CacheTraderInfo {
    pub fn add(&self, value: TraderInfo) {
        let mut cache_lock = self.0.write().unwrap();
        (*cache_lock).push(value);
    }

    pub fn clone_vec(&self) -> Vec<TraderInfo> {
        let lock = self.0.read().unwrap();
        let copy = (*lock).clone();
        drop(lock);
        copy
    }

    pub fn iter(&self) -> IterCacheTraderInfo {
        IterCacheTraderInfo {
            cache: self.clone_vec()
        }
    }
}

pub struct IterCacheTraderInfo {
    cache: Vec<TraderInfo>
}

impl Iterator for IterCacheTraderInfo {
    type Item = TraderInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.len() > 0 {
            Some(self.cache.remove(0))
        } else {
            None
        }
    }
}