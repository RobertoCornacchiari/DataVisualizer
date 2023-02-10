import { useState, useEffect, useMemo } from "react";
import { Routes } from "react-router-dom";
import { Route } from "react-router-dom";
import Graph from "./Components/Graph";
import MarketVisualizer from "./Components/MarketVisualizer";
import Pie from "./Components/Pie";
import Table from "./Components/Table";
import "./index.css";
import { ITraderGood } from "./interfaces";

const App = () => {
  const [data, setData] = useState<ITraderGood[]>([]);

  const [lastData, setLastData] = useState<Map<string, number>>(new Map());

  const [defaultExchange, setDefaultExchange] = useState<Map<string, number>>(
    new Map()
  );

  useEffect(() => {
    let connection = new EventSource("/currentTraderGoods");
    connection.addEventListener("message", (ev) => {
      const msg: ITraderGood = JSON.parse(ev.data);
      msg.time = "" + msg.time;
      console.log("Current trader goods:", msg);
      setData((prev) => [msg, ...prev]);
      setLastData(new Map(lastData.set(msg.kind, msg.quantity)));
    });
    return () => {
      connection.close();
    };
  }, []);

  //Ask the server for the default exchange rates
  useEffect(() => {
    fetch("/defaultExchange")
      .then((data) => data.json())
      .then((body: [string, number][]) => {
        let map: Map<string, number> = new Map();
        body.forEach((data) => {
          map.set(data[0], data[1]);
        });
        setDefaultExchange(map);
      })
      .catch((err) => console.log("Error during the fetch:", err));
  }, []);

  const dataPie = useMemo(() => {
    return Array.from(
      lastData,
      ([key, value]) =>
        ({
          kind: key,
          quantity: value / defaultExchange.get(key)!,
        } as ITraderGood)
    );
  }, [lastData, defaultExchange]);

  return (
    <Routes>
      <Route
        path="/marketController"
        element={
          <>
            <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
              <MarketVisualizer market="BFB" />
              <MarketVisualizer market="RCNZ" />
              <MarketVisualizer market="ZSE" />
            </div>
            <a href="/">
              <button>Go to Events!</button>
            </a>
          </>
        }
      />
      <Route
        path="/"
        element={
          <>
            <div style={{ display: "flex", gap: 12 }}>
              <div
                style={{
                  display: "flex",
                  flexDirection: "column",
                  alignItems: "center",
                }}
              >
                <Table />
              </div>
              <div>
                <Graph
                  data={data}
                  xField="time"
                  yField="quantity"
                  seriesField="kind"
                />
                <Pie data={dataPie} />
                <a href="/marketController">
                  <button>Go to Markets!</button>
                </a>
              </div>
            </div>
          </>
        }
      ></Route>
    </Routes>
  );
};

export default App;
