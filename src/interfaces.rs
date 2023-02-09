use serde::{Deserialize, Serialize};
use unitn_market_2022::good::{good_kind::GoodKind, consts::{DEFAULT_EUR_YEN_EXCHANGE_RATE, DEFAULT_EUR_USD_EXCHANGE_RATE, DEFAULT_EUR_YUAN_EXCHANGE_RATE}};

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

impl TraderGood {
    fn calc_value(&self) -> f32 {
        let exchange = match self.kind {
            GoodKind::EUR => 1.0,
            GoodKind::YEN => DEFAULT_EUR_YEN_EXCHANGE_RATE,
            GoodKind::USD => DEFAULT_EUR_USD_EXCHANGE_RATE,
            GoodKind::YUAN => DEFAULT_EUR_YUAN_EXCHANGE_RATE,
        };
        self.quantity/exchange
    }
}