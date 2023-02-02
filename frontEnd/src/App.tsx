import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";

function App() {
  const [data, setData] = useState<string[]>([]);
  useEffect(() => {
    let a = new EventSource("/index");
    a.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      setData((prev) => [...prev, msg]);
    });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <h1>{data}</h1>
      </header>
    </div>
  );
}

export default App;
