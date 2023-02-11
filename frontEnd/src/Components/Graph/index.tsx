import React, { useMemo } from "react";
import "./index.css";

import { Line } from "@ant-design/plots";
import { ILogMarket, ITraderGood } from "../../interfaces";

interface IProps {
  data: (ILogMarket | ITraderGood)[];
  xField: string;
  yField: string;
  seriesField: string;
}

const COLORS = ["#1979C9", "#D62A0D", "#FAA219", "#00cb00", "#f700ff"];

const Graph = ({ data, xField, yField, seriesField }: IProps) => {
  const config = {
    data,
    xField,
    yField,
    seriesField,
    width: 550,
    height: 290,
    yAxis: {
      label: {
        formatter: (v: string) =>
          `${v}`.replace(/\d{1,3}(?=(\d{3})+$)/g, (s) => `${s},`),
      },
    },
    color: COLORS,
  };

  return (
    <>
      <Line {...config} />
    </>
  );
};

export default Graph;
