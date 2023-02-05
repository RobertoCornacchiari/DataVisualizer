import React, { useEffect, useState } from "react";
import "./App.css";
import { IColumn, ILogEvent } from "./interfaces";
import Table from "./Table";

const columns: IColumn[] = [
  {
    Header: "Events",
    columns: [
      {
        Header: "Day",
        accessor: "time",
      },
      {
        Header: "Event kind",
        accessor: "event.kind",
      },
      {
        Header: "Good kind",
        accessor: "event.good_kind",
      },
      {
        Header: "Market",
        accessor: "market",
      },
      {
        Header: "Quantity",
        accessor: "event.quantity",
      },
      {
        Header: "Price",
        accessor: "event.price",
      },
      {
        Header: "Error",
        accessor: "error",
      },
    ],
  },
];

const App = () => {
  const [data, setData] = useState<ILogEvent[]>([]);
  useEffect(() => {
    let a = new EventSource("/log");
    a.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      console.log(msg);
      setData((prev) => [msg, ...prev]);
    });
  }, []);

  return (
    <div className="App">
      <Table columns={columns} data={data} />
    </div>
  );
};

export default App;
