import { useState, useEffect } from "react";

import { invoke } from "@tauri-apps/api/tauri";

import Search from "./components/Search/";
import Status from "./components/Status";

import "./App.css";

function App() {
	const [activeID, setActiveID] = useState<number>(0);

	function handleDropdownClick(newID: number) {
		if (newID > 0) {
			invoke("cmd_start_client", { appid: newID }).then(() => {
				console.log("Starting..");
				setActiveID(newID);
			});
		}
	}

	useEffect(() => {
		invoke("cmd_request_data").then((response) => {
			invoke("cmd_populate_data", { apps: response }).then(() => {
				console.log("Database Ready.");
			});
		});
	}, []);

	// function handleEnd() {
	// 	invoke("cmd_load_achievements").then((response) => {
	// 		console.log(response);
	// 	});
	// }

	return (
		<>
			<Status appid={activeID} />
			<Search onDropdownClick={handleDropdownClick} />
		</>
	);
}

export default App;
