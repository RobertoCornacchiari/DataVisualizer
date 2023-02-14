import { Routes } from "react-router-dom";
import { Route } from "react-router-dom";
import GoodVisualizer from "./Components/GoodVisualizer";
import Home from "./Components/Home";
import MarketVisualizer from "./Components/MarketVisualizer";
import TraderPicker from "./Components/TraderPicker";
import "./index.css";

const App = () => {
  return (
    <Routes>
      <Route
        path="/goodsController"
        element={
          <>
            <div className="goodGrid">
              <GoodVisualizer good="EUR" />
              <GoodVisualizer good="USD" />
              <GoodVisualizer good="YEN" />
              <GoodVisualizer good="YUAN" />
            </div>
            <a href="/home">
              <button className="button">Events!</button>
            </a>
          </>
        }
      />
      <Route
        path="/marketsController"
        element={
          <>
            <div style={{ display: "flex", flexDirection: "column", gap: 10, marginBottom: 10 }}>
              <MarketVisualizer market="BFB" />
              <MarketVisualizer market="RCNZ" />
              <MarketVisualizer market="ZSE" />
            </div>
            <a href="/home">
              <button className="button">Events!</button>
            </a>
          </>
        }
      />
      <Route
        path="/home"
        element={
          <Home />
        }
      />
      <Route path="/" element={<TraderPicker />}/>
    </Routes>
  );
};

export default App;
