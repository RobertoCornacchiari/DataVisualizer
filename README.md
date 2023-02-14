# DataVisualizer
Data visualizer for Advanced Programming course project

## How to use
In order to display information related to the simulation you must:
1. Include the following dependencies in the __cargo.toml__ file:
```
serde = "1.0.130"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
futures = "0.3.25"
```

2. Every struct that has to be sent in the body has to implement the __Serialize__ trait (`#[derive(serde::Serialize)]`)

3. To send the requests a connection with the server is needed. Use __reqwest__ to create a client instance to perform requests as follow
```
let client = reqwest::Client::new();
```
4. To execute a query
```
let _res = client.post(url).json(&body).send().await;
```
where the _url_ is the endopoint of the API needed, while _body_ is the content that has to be sent (maybe empty, if the request does not expect any data).

5. Mark the functions as __async__, cause of the __await__

6. In order to make the program wait for the APIs to return use the following crates
```
use futures::executor::block_on;
use tokio::runtime::Runtime;

let rt  = Runtime::new().unwrap();
rt.block_on(<async function>);
```
This will force the execution to wait for the APIs to return before continuing the procedure.

## GET Delay
This API returns the number of milliseconds to wait before performing the next event. This can be usefull in order to inspect what's happening to the trader without being overwhelmed by requests.

The endopint of this API is
```
GET /delay
```
### Usage Example
```
let res = client.get("http://localhost:8000/delay").send().await.unwrap();
let value:u64 = res.json::<u64>().await.unwrap();
```

## GET Trader
This API returns (inside a stream) the trader that has been chosen from the visualizer.

The endpoint of this API is
```
GET /traderToUse
```
### Usage Example
```
async fn initialize() -> u8 {
    let mut connection = EventSource::get("http://localhost:8000/traderToUse");
    let mut traderIndex: u8 = 4;

    loop {
        let next = connection.next().await;

        match next {
            Some(content) => match content {
                Ok(ReqEvent::Message(message)) => {
                    traderIndex = message.data.parse::<u8>().unwrap();
                    break;
                }
                Err(err) => panic!("{err}"),
                _ => continue
            },
            None => continue,
        }
    }
    traderIndex
}
```
This function waits for a message containing the trader index, returning it. A common usage of this may be the following: 
inside the main function call __initialize__
```
let run_time = Runtime::new().unwrap();
let res = run_time.block_on(async { initialize().await });

match res {
    0 => {
        ...
    },
    ...
}
```
The API returns:

0. for __Sabin__'s trader;
1. for __Alfredo__'s trader;
2. for __Taras__'s trader;

## POST LogEvent
This API is used to post every action performed by the trader, in order to display them inside the table. The actions that can be sent to this API are the ones defined in the __EventKind__ enum.

The endpoint of this API is
```
POST /log
```
If running on local the complete url is
```
http://localhost:8000/log
```
The body to pass is an instance of the stuct __LogEvent__
```
use serde::{Deserialize, Serialize};

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
    pub time: u32,
    pub event: CustomEvent,
    pub market: String,
    pub result: bool,
    pub error: Option<String>
}
```
(the structs/enums defined in the market-common could not be used because they do not implement the __Serialize__ trait)
The attribute _time_ can be omitted (it is computed server-side); same for the _error_ (if not present it will be considered _None_). The _error_ should be populated with the error returned by the market in case of rejection.

### Usage Example[^1]
```
pub fn craft_log_event(time: u32, kind: CustomEventKind, good_kind: GoodKind, quantity: f32, price: f32, market: String, result: bool, error: Option<String>) -> LogEvent {
    let custom_ev = CustomEvent {
        kind,
        good_kind,
        quantity,
        price,
    };
    LogEvent {
        market,
        result,
        error,
        time,
        event: custom_ev,
    }
}

...

let goodkind: GoodKind = GoodKind::YEN;
let quantity: f32 = 12.3;
let price: f32 = 42.0;
let market_name: String = "BFB".to_string();
let e_string: String = "NonPositiveQuantity".to_string();

let _res = client.post("http://localhost:8000/log").json(&craft_log_event(CustomEventKind::LockedBuy, goodkind, quantity, price, market_name, false, Some(e_string))).send().await;
```

## POST market status
This API is used to send the current status of a market (quantity of goods, exchange_buy/sell_rate), in order to produce graphs.
The endpoint of this API is
```
POST /currentGoodLabels/<market>
```
where market is one of the market used in the simulationo (`"BFB", "RCNZ", "ZSE"`).

The body to pass is a `Vec<GoodLabel>`, easily obtainable calling the method _get\_goods()_ of the Market trait.

This API should be called at the beginning and after each __Event__ performed __WITH SUCCESS__ by the trader (every time a day passes).

### Usage Example[^1]
```
let labels: Vec<GoodLabel> = market.borrow().get_goods();

let _res = client.post("http://localhost:8000/currentGoodLabels/".to_string() + market_name).json(&labels).send().await;
```

## POST market status
This API is used to send the current status of the trader (quantity of each good).
The endpoint of this API is
```
POST /traderGoods
```
The body to pass is a `Vec<TraderGood>`, where
```
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TraderGood{
    kind: GoodKind,
    quantity: f32
}

```

This API should be called at the beginning and after each __Event__ performed __WITH SUCCESS__ by the trader (every time a day passes).

### Usage Example[^1]
```
let mut tradergoods = vec![];

tradergoods.push(TraderGood{kind: GoodKind::EUR, quantity: self.cash});

for goodkind in &self.goods{
    tradergoods.push(TraderGood{
        kind: goodkind.borrow().get_kind().clone(),
        quantity: goodkind.borrow().get_qty()
    });
}

let _res = client.post("http://localhost:8000/traderGoods").json(&tradergoods).send().await;
```

[^1]: example based on the code of Andone Sabin.