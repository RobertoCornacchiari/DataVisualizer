import React, { useEffect, useState } from "react";
import { Routes } from "react-router-dom";
import { Route } from "react-router-dom";
import MarketVisualizer from "./Components/MarketVisualizer";
import "./index.css";
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
    <Routes>
      <Route
        path="/marketController"
        element={
          <>
            <a href="/">
              <button>Go to Events!</button>
            </a>
            <MarketVisualizer market="BFB" />
            <MarketVisualizer market="RCNZ" />
            <MarketVisualizer market="ZSE" />
          </>
        }
      />
      <Route
        path="/"
        element={
          <>
            <Table columns={columns} data={data} />
            <a href="/marketController">
              <button>Go to Markets!</button>
            </a>
          </>
        }
      ></Route>
    </Routes>
  );
};

export default App;
