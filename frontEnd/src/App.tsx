import { useState, useEffect, useMemo } from "react";
import { Routes } from "react-router-dom";
import { Route } from "react-router-dom";
import Graph from "./Components/Graph";
import MarketVisualizer from "./Components/MarketVisualizer";
import Table from "./Components/Table";
import "./index.css";
import { ITraderGood } from "./interfaces";

import { Pie } from "@ant-design/plots";

const App = () => {
  const [data, setData] = useState<ITraderGood[]>([]);

  const [lastData, setLastData] = useState<Map<string, number>>(new Map());

  useEffect(() => {
    let connection = new EventSource("/currentTraderGoods");
    connection.addEventListener("message", (ev) => {
      const msg: ITraderGood = JSON.parse(ev.data);
      console.log(msg);
      setData((prev) => [msg, ...prev]);
      setLastData(new Map(lastData.set(msg.kind, msg.quantity)));
    });
    return () => {
      connection.close();
    };
  }, []);

  const config = useMemo(() => {
    return {
      appendPadding: 10,
      data: Array.from(lastData, ([key, value]) => ({
        kind: key,
        quantity: value,
      })),
      angleField: "quantity",
      colorField: "kind",
      radius: 0.8,
      label: {
        type: "outer",
        content: "{name} {percentage}",
      },
      interactions: [
        {
          type: "pie-legend-active",
        },
        {
          type: "element-active",
        },
      ],
    };
  }, [lastData]);

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
              <br />
              <Pie {...config} />
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
