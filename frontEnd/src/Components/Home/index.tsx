
import Table from "../Table";
import TraderInfo from "../TraderInfo";

const Home = () => {

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
      <div>
        <TraderInfo />
        <a href="/marketController">
          <button>Go to Markets!</button>
        </a>
      </div>
    </div>
  );
};

export default Home;
