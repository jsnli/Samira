import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import Search from "./components/Search/";
import Status from "./components/Status";
import List from "./components/List";

import "./App.css";

interface Achievement {
	api_name: string;
	name: string;
	desc: string;
	status: boolean;
}

function App() {
	const [activeID, setActiveID] = useState<number>(0);
	const [achievements, setAchievements] = useState<Achievement[]>([]);

	useEffect(() => {
		invoke("cmd_request_data").then((response) => {
			invoke("cmd_populate_data", { apps: response }).then(() => {
				console.log("Database Ready.");
			});
		});
	}, []);

	function handleDropdownClick(newID: number) {
		if (newID > 0) {
			invoke("cmd_start_client", { appid: newID }).then(() => {
				console.log("Starting..");
				setActiveID(newID);
				LoadAchievements();
			});
		}
	}

	function LoadAchievements() {
		console.log("Loading,,");
		invoke("cmd_load_achievements").then((response) => {
			/* console.log(response); */
			const data = response as Achievement[];
			setAchievements(data);
		});
	}

	return (
		<>
			<div className="sidebar">
				<Status appid={activeID} />
				<Search onDropdownClick={handleDropdownClick} />
			</div>
			<div className="main">
				<List items={achievements} /> 
			</div>
			

		</>
	);
}

export default App;
