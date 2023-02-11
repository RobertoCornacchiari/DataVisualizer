import { Routes } from "react-router-dom";
import { Route } from "react-router-dom";
import Home from "./Components/Home";
import MarketVisualizer from "./Components/MarketVisualizer";
import "./index.css";

const App = () => {
  return (
    <Routes>
      <Route
        path="/marketController"
        element={
          <>
            <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
              <MarketVisualizer market="BFB" />
              <MarketVisualizer market="RCNZ" />
              <MarketVisualizer market="ZSE" />
            </div>
            <a href="/">
              <button>Go to Events!</button>
            </a>
          </>
        }
      />
      <Route
        path="/"
        element={
          <Home />
        }
      ></Route>
    </Routes>
  );
};

export default App;
