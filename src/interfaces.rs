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
