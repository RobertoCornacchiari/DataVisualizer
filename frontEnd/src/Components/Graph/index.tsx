import React, { useEffect, useMemo, useState } from "react";
import "./index.css";

import { Line } from "@ant-design/plots";
import { ILogMarket } from "../../interfaces";

interface IProps {
  data: ILogMarket[];
}

const COLORS = ["#1979C9", "#D62A0D", "#FAA219", "#00cb00"];

const Graph = ({ data }: IProps) => {

  const config = useMemo(() => {
    console.log(data);
    return {
      data,
      xField: "time",
      yField: "value",
      seriesField: "kind",
      width: 500,
      yAxis: {
        label: {
          formatter: (v: string) =>
            `${v}`.replace(/\d{1,3}(?=(\d{3})+$)/g, (s) => `${s},`),
        },
      },
      color: COLORS,
    };
  }, [data]);

  return (
    <>
      <Line {...config} />
    </>
  );
};

export default Graph;
