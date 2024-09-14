import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import Search from "./components/Search/";
import Status from "./components/Status";

import "./App.css";

function App() {
  const [activeID, setActiveID] = useState<number>(0)

	function handleDropdownClick(newID: number) {
		setActiveID(newID);
	}
		
  useEffect(() => {
    invoke("cmd_request_data").then((response) => {
      invoke("cmd_populate_data", { apps: response }).then(() => {
        console.log("Database Ready.");
      });
    });
  }, []);

	function handleStart() {
		invoke("cmd_start").then(() => {
			console.log("Starting..");
		});
	}


	function handleEnd() {
		invoke("cmd_get_info").then(() => {
			console.log("End..");
		});
	}

  return (
    <>
      <Search onDropdownClick={handleDropdownClick} />
			<Status appid={activeID} />
			<button onClick={handleStart}>start</button>
<button onClick={handleEnd}>end</button>

			
    </>
  );
}

export default App;
