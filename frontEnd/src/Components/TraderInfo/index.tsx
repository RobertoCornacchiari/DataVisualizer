import { useEffect, useMemo, useRef, useState } from "react";
import { ITraderGood } from "../../interfaces";
import Graph from "../Graph";
import Pie from "../Pie";

const COLORS = ["#1979C9", "#D62A0D", "#FAA219", "#00cb00", "#f700ff"];

const TraderInfo = () => {
  const [data, setData] = useState<ITraderGood[]>([]);

  const [lastData, setLastData] = useState<Map<string, number>>(new Map());

  const walletValue = useRef(0);

  const [defaultExchange, setDefaultExchange] = useState<Map<string, number>>(
    new Map()
  );

  const [size, setSize] = useState<number>(0);

  const filter = (data: ITraderGood[], size: number): ITraderGood[] => {
    if (size === 0) return data;
    //The 5 is there because there are 4 goodkinds + total
    else {
      let a = data.slice(-Math.min(5 * size, data.length));
      console.log("Slice:", a);
      return a;
    }
  };

  useEffect(() => {
    let connection = new EventSource("/currentTraderGoods");

    connection.addEventListener("message", (ev) => {
      const msg: ITraderGood = JSON.parse(ev.data);
      msg.time = "" + msg.time;
      console.log("Current trader goods:", msg);
      setData((prev) => [...prev, msg]);
      setLastData((l) => new Map(l.set(msg.kind, msg.quantity)));
      if (msg.kind === "TOT") walletValue.current = msg.quantity;
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
    <>
      <Graph
        data={filter(data, size)}
        xField="time"
        yField="quantity"
        seriesField="kind"
        colors={COLORS}
        width={500}
      />
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
      <br />
      <b>Wallet value:</b> {walletValue.current.toFixed(4)} EUR
      <Pie data={dataPie} />
    </>
  );
};

export default TraderInfo;
