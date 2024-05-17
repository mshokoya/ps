import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

function App() {
  console.log("App fc");
  useEffect(() => {
    const waitQueue = listen("waitQueue", (event) => {
      console.log("IN DA WaitQueue");
      console.log(event.payload);
    });

    const processQueue = listen("processQueue", (event) => {
      console.log("IN DA ProcessQueue");
      console.log(event.payload);
    });

    const timeoutQueue = listen("timeoutQueue", (event) => {
      console.log("IN DA timeoutQueue");
      console.log(event.payload);
    });

    // https://github.com/tauri-apps/tauri/discussions/5194
    return () => {
      waitQueue.then((f) => f());
      processQueue.then((f) => f());
      timeoutQueue.then((f) => f());
    };
  }, []);

  const handleDemine = () => {
    invoke("check_task", {
      args: {
        account_id: "67e55044-10b1-426f-9247-bb680e5fe0c8",
        timeout: {
          time: 5000,
          rounds: 2,
        },
      },
    });
  };

  return (
    <div className="container">
      <button onClick={() => handleDemine()}></button>
    </div>
  );
}

export default App;
