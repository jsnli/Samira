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
	const [achievements, setAchievements] = useState<Achievement[]>([]);
	const [statusMessage, setStatusMessage] =
		useState<string>("Loading database.");

	useEffect(() => {
		invoke("cmd_request_data").then((response) => {
			invoke("cmd_populate_data", { apps: response }).then(() => {
				setStatusMessage("Database ready.");
			});
		});
	}, []);

	function handleDropdownClick(newID: number) {
		if (newID > 0) {
			invoke("cmd_start_client", { appid: newID }).then((response) => {
				if (response) {
					setStatusMessage("Starting client.");
					LoadAchievements();
				} else {
					setStatusMessage("Error loading client. Steam must be running and you must own the game selected.");
				}
			});
		}
	}

	function LoadAchievements() {
		invoke("cmd_load_achievements").then((response) => {
			/* console.log(response); */
			const data = response as Achievement[];
			setAchievements(data);
			setStatusMessage("Achievements loaded.");
		});
	}

	return (
		<>
			<div className="sidebar">
				<Search onDropdownClick={handleDropdownClick} />
				<Status message={statusMessage} />
			</div>
			<div className="main">
				<List achievements={achievements} />
			</div>
		</>
	);
}

export default App;
