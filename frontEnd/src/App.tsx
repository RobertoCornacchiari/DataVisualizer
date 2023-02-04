import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import Table, { IColumn } from "./Table";

const columns: IColumn[]  = [
  {
    Header: "Events",
    columns: [
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
        accessor: "market"
      },
      {
        Header: "Quantity",
        accessor: "event.quantity"
      },
      {
        Header: "Price",
        accessor: "event.price"
      },
      {
        Header: "Error",
        accessor: "error"
      }
    ]
  }
]

function App() {
  const [data, setData] = useState<any[]>([]);
  useEffect(() => {
    let a = new EventSource("/log");
    a.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      console.log(msg);
      setData((prev) => [...prev, msg]);
    });
  }, []);

  return (
    <div className="App">
      <Table columns={columns} data={data}/>
    </div>
  );
}

export default App;
