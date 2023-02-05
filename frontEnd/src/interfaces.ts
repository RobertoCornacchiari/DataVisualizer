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
