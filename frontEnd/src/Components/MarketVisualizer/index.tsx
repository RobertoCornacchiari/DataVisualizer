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

  const [size, setSize] = useState<number>(0);

  const filter = (data: ILogMarket[], size: number): ILogMarket[] => {
    if (size === 0) return data;
    //The 4 is there because there are 4 goodkinds
    else {
      let a = data.slice(-Math.min(4 * size, data.length));
      console.log("Slice:", a);
      return a;
    }
  };

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
      msg.log.time = "" + msg.log.time;
      console.log("Market:", market, "\nChannel: ", msg.channel, "\n", msg);
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
    <div className="boxGraph">
      <div className="graphContainer">
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
          }}
        >
          <h3>{market}</h3>
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
        <div>
          Goods quantity
          <Graph
            data={filter(dataCurrentGoods, size)}
            xField="time"
            yField="value"
            seriesField="kind"
          />
        </div>
        <div>
          Exchange buy rate
          <Graph
            data={filter(dataCurrentBuyRate, size)}
            xField="time"
            yField="value"
            seriesField="kind"
          />
        </div>
        <div>
          Exchange sell rate
          <Graph
            data={filter(dataCurrentSellRate, size)}
            xField="time"
            yField="value"
            seriesField="kind"
          />
        </div>
      </div>
    </div>
  );
};

export default MarketVisualizer;
