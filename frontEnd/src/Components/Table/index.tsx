import { useState, useEffect, useRef } from "react";
import { useTable, usePagination } from "react-table";
import { IColumn, ILogEvent } from "../../interfaces";
import "./index.css";

const columns: IColumn[] = [
  {
    Header: "Events",
    columns: [
      {
        Header: "Day",
        accessor: "time",
      },
      {
        Header: "Event kind",
        accessor: "event.kind",
      },
      {
        Header: "Good kind",
        accessor: "event.good_kind",
      },
      {
        Header: "Market",
        accessor: "market",
      },
      {
        Header: "Quantity",
        accessor: "event.quantity",
      },
      {
        Header: "Price",
        accessor: "event.price",
      },
      {
        Header: "Error",
        accessor: "error",
      },
    ],
  },
];

const Table = () => {
  const [data, setData] = useState<ILogEvent[]>([]);
  const [stop, setStop] = useState<boolean>(false);
  const [dataBuffer, setDataBuffer] = useState<ILogEvent[]>([]);

  //Function that redirects the messages to the correct state, based on the stop status
  const receiveData = useRef((msg: ILogEvent) => {
    if (!stop) {
      setData((prev) => [msg, ...prev]);
    } else {
      setDataBuffer((prev) => [msg, ...prev]);
    }
  });

  useEffect(() => {
    let connection = new EventSource("/log");
    connection.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      console.log(msg);
      receiveData.current(msg);
    });
    return () => {
      connection.close();
    };
  }, []);

  //Mechanism used for client-side pause and resume
  useEffect(() => {
    //Update the function that sends the incoming messages to the corret state
    receiveData.current = (msg: ILogEvent) => {
      if (!stop) {
        setData((prev) => [msg, ...prev]);
      } else {
        setDataBuffer((prev) => [msg, ...prev]);
      }
    };

    //Resume the unseen messages
    if (!stop) setData((prev) => [...dataBuffer, ...prev]);
  }, [stop]);

  // Use the state and functions returned from useTable to build the UI
  const {
    getTableProps,
    getTableBodyProps,
    headerGroups,
    page,
    prepareRow,
    canPreviousPage,
    canNextPage,
    pageOptions,
    pageCount,
    gotoPage,
    nextPage,
    previousPage,
    setPageSize,
    state: { pageIndex, pageSize },
  } = useTable(
    {
      columns,
      data,
      initialState: { pageIndex: 0, pageSize: 10 },
    },
    usePagination
  );
  // Render the UI
  return (
    <>
      <table {...getTableProps()}>
        <thead>
          {headerGroups.map((headerGroup) => (
            <tr {...headerGroup.getHeaderGroupProps()}>
              {headerGroup.headers.map((column) => (
                <th {...column.getHeaderProps()}>{column.render("Header")}</th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody {...getTableBodyProps()}>
          {page.map((row, i) => {
            prepareRow(row);
            let classes = row.original.result ? "valid" : "error";
            return (
              <tr {...row.getRowProps()}>
                {row.cells.map((cell) => {
                  return (
                    <td {...cell.getCellProps()} className={classes}>
                      {cell.render("Cell")}
                    </td>
                  );
                })}
              </tr>
            );
          })}
        </tbody>
      </table>
      <div className="button_container">
        <button
          className="button"
          onClick={() => gotoPage(0)}
          disabled={!canPreviousPage}
        >
          {"<<"}
        </button>{" "}
        <button
          className="button"
          onClick={() => previousPage()}
          disabled={!canPreviousPage}
        >
          {"<"}
        </button>{" "}
        <button
          className="button"
          onClick={() => nextPage()}
          disabled={!canNextPage}
        >
          {">"}
        </button>{" "}
        <button
          className="button"
          onClick={() => gotoPage(pageCount - 1)}
          disabled={!canNextPage}
        >
          {">>"}
        </button>{" "}
        <span>
          Page{" "}
          <strong>
            {pageIndex + 1} of {pageOptions.length}
          </strong>{" "}
        </span>
        <select
          value={pageSize}
          onChange={(e) => {
            setPageSize(Number(e.target.value));
          }}
          className="select"
        >
          {[10, 20, 30, 40, 50].map((pageSize) => (
            <option key={pageSize} value={pageSize}>
              Show {pageSize}
            </option>
          ))}
        </select>
        <button
          className="button"
          onClick={() => setData([])}
          disabled={data.length === 0}
        >
          DEL
        </button>
      </div>
    </>
  );
};

export default Table;
