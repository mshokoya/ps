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
        // accountID: "hujnkhyughbjkih",
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
