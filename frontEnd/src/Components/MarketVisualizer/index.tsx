import React from "react";
import Graph from "../Graph";
import './index.css';

interface IProps {
  market: string;
}

const MarketVisualizer = ({ market }: IProps) => {
  return (
    <div className="graphContainer">
      <Graph path={`currentGoods/${market}`} />
      <Graph path={`currentBuyRate/${market}`} />
      <Graph path={`currentSellRate/${market}`} />
    </div>
  );
};

export default MarketVisualizer;
