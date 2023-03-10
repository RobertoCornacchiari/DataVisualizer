import { useState } from "react";
import Table from "../Table";
import TraderInfo from "../TraderInfo";

const Home = () => {
  const [stop, setStop] = useState<boolean>(false);

  const handle_block = () => {
    fetch("/block", {
      method: "POST",
    })
      .then(() => {
        console.log("Successful block");
        setStop(true);
      })
      .catch((err) => console.log("Error during blocking:", err));
  };

  const handle_unblock = () => {
    fetch("/unblock", {
      method: "POST",
    })
      .then(() => {
        console.log("Successful unblock");
        setStop(false);
      })
      .catch((err) => console.log("Error during unblocking:", err));
  };

  const handle_click = (stop: boolean) => {
    if (stop) {
      handle_unblock();
    } else {
      handle_block();
    }
  };

  const [value, setValue] = useState<number>(1000);

  const handleChange = ({ target }: { target: any }) => {
    const { value } = target;
    setValue(value);
  };

  const handleOver = () => {
    fetch("/delay", {
      method: "POST",
      headers: {
        "Content-type": "application/json",
      },
      body: JSON.stringify(value),
    })
      .then(() => console.log("Delay successfully updated"))
      .catch((err) => console.log("Error during saving of delay:", err));
  };

  return (
    <div style={{ display: "flex", gap: 12 }}>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          gap: 12,
        }}
      >
        <Table />
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          gap: 12,
        }}
      >
        <TraderInfo />
        Delay: {value}ms
        <input
          type="range"
          min="10"
          value={value}
          max="10000"
          className="slider"
          onChange={handleChange}
          onMouseUp={handleOver}
        />
        <div style={{ display: "flex", gap: 12 }}>
          <a href="/marketsController">
            <button className="button">Markets!</button>
          </a>
          <a href="/goodsController">
            <button className="button">Goods!</button>
          </a>
          <button className="button" onClick={() => handle_click(stop)}>
            {stop ? "RESUME" : "PAUSE"}
          </button>
        </div>
      </div>
    </div>
  );
};

export default Home;
