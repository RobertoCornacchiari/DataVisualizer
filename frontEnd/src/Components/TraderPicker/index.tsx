import { useNavigate } from "react-router-dom";
import "./index.css";

const Traders = [
  {
    name: "Andone Sabin",
  },
  {
    name: "Bombace Alfredo",
  },
  {
    name: "Rashkevych Taras",
  },
];

const TraderPicker = () => {
  let navigate = useNavigate();

  const choseTrader = (value: number) => {
    fetch("/traderToUse", {
      method: "POST",
      body: JSON.stringify(value),
    })
      .then(() => {
        navigate("/home");
      })
      .catch((err) => console.log("Error during trader selection:", err));
  };

  return (
    <>
      <div className="traderContainer">
        {Traders.map((trader, index) => {
          return (
            <button className="traderButton" onClick={() => choseTrader(index)}>
              {trader.name}
            </button>
          );
        })}
      </div>
    </>
  );
};

export default TraderPicker;
