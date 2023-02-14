import React, { useEffect, useState } from "react";
import { IGoodInfo } from "../../interfaces";
import Graph from "../Graph";

interface IProps {
  good: string;
}

const COLORS = ["#008cff", "#014882", "#D62A0D", "#94610f", "#00cb00" , "#016501"];

const GoodVisualizer = ({ good }: IProps) => {
  const [data, setData] = useState<IGoodInfo[]>([]);

  const [size, setSize] = useState<number>(0);

  const filter = (data: IGoodInfo[], size: number): IGoodInfo[] => {
    if (size === 0) return data;
    //The 6 is there because there are 6 different possibilities (buy, sell for each market)
    else {
      let a = data.slice(-Math.min(6 * size, data.length));
      console.log("Slice:", a);
      return a;
    }
  };

  useEffect(() => {
    let connection = new EventSource("goodInfo/" + good);

    connection.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      msg.time = "" + msg.time;
      console.log(msg);
      setData((prev) => [...prev, msg]);
    });

    return () => {
      connection.close();
    };
  }, []);

  return (
    <div className="boxGraph">
      <div className="graphContainer">
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
          }}
        >
          <h3>{good}</h3>
          <select
            value={size}
            onChange={(e) => {
              setSize(Number(e.target.value));
            }}
            className="select"
          >
            {["All", 10, 25, 50, 100].map((pageSize) => (
              <option key={pageSize} value={pageSize}>
                Show {pageSize}
              </option>
            ))}
          </select>
        </div>
        <Graph
          data={filter(data, size)}
          xField="time"
          yField="value"
          seriesField="data"
          colors={COLORS}
          width={700}
        />
      </div>
    </div>
  );
};

export default GoodVisualizer;
