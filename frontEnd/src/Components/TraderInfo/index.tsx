import { useEffect, useMemo, useRef, useState } from "react";
import { ITraderGood } from "../../interfaces";
import Graph from "../Graph";
import Pie from "../Pie";

const TraderInfo = () => {
  const [data, setData] = useState<ITraderGood[]>([]);

  const [lastData, setLastData] = useState<Map<string, number>>(new Map());

  const walletValue = useRef(0);

  const [defaultExchange, setDefaultExchange] = useState<Map<string, number>>(
    new Map()
  );

  useEffect(() => {
    let connection = new EventSource("/currentTraderGoods");

    connection.addEventListener("message", (ev) => {
      const msg: ITraderGood = JSON.parse(ev.data);
      msg.time = "" + msg.time;
      console.log("Current trader goods:", msg);
      setData((prev) => [...prev, msg]);
      setLastData(new Map(lastData.set(msg.kind, msg.quantity)));
      if (msg.kind === "TOT")
        walletValue.current = msg.quantity;
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
      <Graph data={data} xField="time" yField="quantity" seriesField="kind" />
      <br />
      <b>Wallet value:</b> {walletValue.current.toFixed(4)} EUR
      <Pie data={dataPie} />
    </>
  );
};

export default TraderInfo;
