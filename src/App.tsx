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

interface Info {
	app_id: number;
	app_name: string;
	user_id: number;
	user_name: string;
}

function App() {
	const [achievements, setAchievements] = useState<Achievement[]>([]);
	const [statusMessage, setStatusMessage] =
		useState<string>("Loading database.");
	const [info, setInfo] = useState<Info>({
		app_id: 0,
		app_name: "",
		user_id: 0,
		user_name: "",
	});

	useEffect(() => {
		invoke("cmd_request_data").then((response) => {
			invoke("cmd_populate_data", { apps: response }).then(() => {
				setStatusMessage("Database ready.");
			});
		});
	}, []);

	function handleDropdownClick(newID: number, newName: string) {
		if (newID > 0) {
			invoke("cmd_start_client", { appid: newID }).then((response) => {
				if (response) {
					setStatusMessage("Starting client.");
					LoadAchievements();
					LoadStatistics(newID);
					UpdateStatusInfo(newID, newName);
				} else {
					setStatusMessage(
						"Error loading client. Steam must be running and you must own the game selected.",
					);
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

	function LoadStatistics(newID: number) {
		invoke("cmd_load_statistics", { appid: newID }).then((response) => {
			console.log(response);
		});
	}

	function UpdateStatusInfo(appid: number, appname: string) {
		invoke("cmd_retrieve_user").then((response) => {
			console.log(response);
			interface User {
				user_steam_id: number;
				user_name: string;
			}
			const user_data = response as User;
			setInfo({
				app_id: appid,
				app_name: appname,
				user_id: user_data.user_steam_id,
				user_name: user_data.user_name,
			})
		});
	}

	return (
		<>
			<div className="sidebar">
				<Search onDropdownClick={handleDropdownClick} />
				<Status message={statusMessage} info={info} />
			</div>
			<div className="main">
				<List achievements={achievements} />
			</div>
		</>
	);
}

export default App;
