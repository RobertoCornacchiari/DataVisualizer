import React, { useEffect, useState } from "react";
import { Channels, ILogMarket, IMsgMultiplexed } from "../../interfaces";
import Graph from "../Graph";
import "./index.css";

interface IProps {
  market: string;
}

const MarketVisualizer = ({ market }: IProps) => {
  const [dataCurrentGoods, setDataCurrentGoods] = useState<ILogMarket[]>([]);
  const [dataCurrentBuyRate, setDataCurrentBuyRate] = useState<ILogMarket[]>(
    []
  );
  const [dataCurrentSellRate, setDataCurrentSellRate] = useState<ILogMarket[]>(
    []
  );

  useEffect(() => {
    let connection = new EventSource("currentMarket/" + market);
    let retryTime = 1;

    //Trova il modo di convertire direttamente all'interfaccia corretta, attualmente step intermedio con doppia stringa

    connection.addEventListener("message", (ev) => {
      let received: { channel: string; log: string } = JSON.parse(ev.data);
      let msg: IMsgMultiplexed = {
        channel: received.channel as Channels,
        log: JSON.parse(received.log),
      };
      console.log(msg);
      switch (msg.channel) {
        case Channels.CurrentGoods:
          setDataCurrentGoods((prev) => [...prev, msg.log]);
          break;
        case Channels.CurrentBuyRate:
          setDataCurrentBuyRate((prev) => [...prev, msg.log]);
          break;
        case Channels.CurrentSellRate:
          setDataCurrentSellRate((prev) => [...prev, msg.log]);
          break;
        default:
          console.log("Nope");
          break;
      }
    });

    connection.addEventListener("open", () => {
      retryTime = 1;
    });

    connection.addEventListener("error", () => {
      connection.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(
        () => (connection = new EventSource("currentMarket/" + market)),
        (() => timeout * 1000)()
      );
    });

    return () => {
      console.log("Chiudi connessione");
      connection.close();
    };
  }, []);

  return (
    <div className="graphContainer">
      <Graph
        data={dataCurrentGoods}
        xField="time"
        yField="value"
        seriesField="kind"
      />
      <Graph
        data={dataCurrentBuyRate}
        xField="time"
        yField="value"
        seriesField="kind"
      />
      <Graph
        data={dataCurrentSellRate}
        xField="time"
        yField="value"
        seriesField="kind"
      />
    </div>
  );
};

export default MarketVisualizer;
