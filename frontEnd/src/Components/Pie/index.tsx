import { Pie as PieChart } from "@ant-design/plots";
import { useMemo } from "react";
import { ITraderGood } from "../../interfaces";

interface IProps {
  data: ITraderGood[];
}

const Pie = ({ data }: IProps) => {
  const config = useMemo(() => {
    return {
      appendPadding: 10,
      data,
      angleField: "quantity",
      colorField: "kind",
      radius: 0.8,
      label: {
        type: "outer",
        content: "{name} {percentage}",
        style: { fill: "white" },
      },
      interactions: [
        {
          type: "pie-legend-active",
        },
        {
          type: "element-active",
        },
      ],
    };
  }, [data]);
  return <PieChart {...config} />;
};

export default Pie;
