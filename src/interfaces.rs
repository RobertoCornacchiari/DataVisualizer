use serde::{Deserialize, Serialize};
use unitn_market_2022::good::good_kind::GoodKind;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomEventKind {
    Bought, Sold, LockedBuy, LockedSell, Wait
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
    pub event: CustomEvent,
    pub market: String,
    pub result: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>
}

pub const MARKETS:[&str; 3] =["BFB","RCNZ","ZSE"];