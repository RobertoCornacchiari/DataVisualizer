import React, { useEffect, useMemo, useState } from "react";
import "./index.css";

import { Line } from "@ant-design/plots";

interface ILogMarket {
  value: number;
  time: number;
  kind: string;
}

interface IProps {
  path: string;
}

const COLORS = ["#1979C9", "#D62A0D", "#FAA219", "#00cb00"];

const Graph = ({ path }: IProps) => {
  const [dataGraph, setDataGraph] = useState<ILogMarket[]>([]);

  useEffect(() => {
    let connection = new EventSource(path);
    let retryTime = 1;
    console.log("Tentativo per path: ", path);
    connection.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      console.log(msg);
      setDataGraph((prev) => [...prev, msg]);
    });

    connection.addEventListener("open", () => {
      console.log("Successo per path: ", path);
      retryTime = 1;
    });

    connection.addEventListener("error", () => {
      console.log("Errore path:", path);
      connection.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(
        () => (connection = new EventSource(path)),
        (() => timeout * 1000)()
      );
    });
  }, []);

  const config = useMemo(() => {
    console.log(dataGraph);
    return {
      data: dataGraph,
      xField: "time",
      yField: "value",
      seriesField: "kind",
      width: 500,
      yAxis: {
        label: {
          formatter: (v: string) =>
            `${v}`.replace(/\d{1,3}(?=(\d{3})+$)/g, (s) => `${s},`),
        },
      },
      color: COLORS,
    };
  }, [dataGraph]);

  return (
    <>
      <Line {...config} />
    </>
  );
};

export default Graph;
