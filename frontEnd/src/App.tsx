import { useState, useEffect } from "react";
import { Routes } from "react-router-dom";
import { Route } from "react-router-dom";
import Graph from "./Components/Graph";
import MarketVisualizer from "./Components/MarketVisualizer";
import Table from "./Components/Table";
import "./index.css";
import { ITraderGood } from "./interfaces";

const App = () => {
  const [data, setData] = useState<ITraderGood[]>([]);
  useEffect(() => {
    let connection = new EventSource("/currentTraderGoods");
    connection.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      console.log(msg);
      setData((prev) => [msg, ...prev]);
    });
    return () => {
      connection.close();
    };
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
            <div style={{ display: "flex", gap: 12 }}>
              <div>
                <Table />
              </div>
              <Graph
                data={data}
                xField="time"
                yField="quantity"
                seriesField="kind"
              />
            </div>{" "}
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
