import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

// total_page_scrape: u8,
// scrape_time: Option<Timestamp>,
// list_name: String,
// scrape_id: Uuid,

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
    invoke("confirm_task", {
      args: {
        account: {
          id: crypto.randomUUID(),
          domain: "ljskdad",
          trialTime: new Date().getTime(),
          suspended: false,
          loginType: "gmail",
          verified: true,
          email: "mikkey_g@hotmail.co.uk",
          password: "golsdsas",
          proxy: "fesdsfdsdsf",
          creditsUsed: 54,
          creditLimit: 32,
          renewalDate: new Date().getTime(),
          renewalStartDate: new Date().getTime(),
          renewalEndDate: new Date().getTime(),
          trialDaysLeft: new Date().getTime(),
          lastUsed: new Date().getTime(),
          cookies: "dsfkdsjkhlajs;kfgjlhjkdkasfjgljfhdkasl;",
          history: {
            totalPageScrape: 16,
            scrapeTime: new Date().getTime(),
            listName: "the lst",
            scrapeID: crypto.randomUUID(),
          },
        },
        timeout: crypto.randomUUID(),
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
