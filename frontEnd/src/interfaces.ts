export interface IColumn {
  Header: string;
  columns: {
    Header: string;
    accessor: string;
  }[];
}

export interface ILogEvent {
  time: number;
  event: {
    kind: string;
    good_kind: string;
    quantity: number;
    price: number;
  };
  market: string;
  result: boolean;
  error: string;
}

export interface ILogMarket {
  value: number;
  time: string;
  kind: string;
}

export enum Channels {
  CurrentGoods = "CurrentGoods", CurrentBuyRate = "CurrentBuyRate", CurrentSellRate = "CurrentSellRate"
}

export interface IMsgMultiplexed {
  channel: Channels;
  log: ILogMarket;
}

export interface ITraderGood {
  time: string;
  kind: string;
  quantity: number;
}