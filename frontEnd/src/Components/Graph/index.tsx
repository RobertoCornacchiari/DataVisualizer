import React, { useMemo } from "react";
import "./index.css";

import { Line } from "@ant-design/plots";
import { IGoodInfo, ILogMarket, ITraderGood } from "../../interfaces";

interface IProps {
  data: (ILogMarket | ITraderGood | IGoodInfo)[];
  xField: string;
  yField: string;
  seriesField: string;
  colors: string[];
  width: number,
}

const Graph = ({ data, xField, yField, seriesField, colors, width }: IProps) => {
  const config = {
    data,
    xField,
    yField,
    seriesField,
    width,
    height: 290,
    legend: {
      maxRow: 2
    },
    yAxis: {
      label: {
        formatter: (v: string) =>
          `${v}`.replace(/\d{1,3}(?=(\d{3})+$)/g, (s) => `${s},`),
      },
    },
    color: colors,
  };

  return (
    <>
      <Line {...config} />
    </>
  );
};

export default Graph;
