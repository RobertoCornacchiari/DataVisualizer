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
        console.log("Successful block");
        setStop(false);
      })
      .catch((err) => console.log("Error during blocking:", err));
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
        <div style={{ display: "flex", gap: 12 }}>
          <a href="/marketController">
            <button className="button">Go to Markets!</button>
          </a>
          <button
            className="button"
            onClick={stop ? handle_unblock : handle_block}
          >
            {stop ? "RESUME" : "PAUSE"}
          </button>
        </div>
      </div>
    </div>
  );
};

export default Home;
