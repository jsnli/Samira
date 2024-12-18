import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

import Search from "./components/Search";
import Status from "./components/Status";
import Tabs from "./components/Tabs";
import AchievementView from "./components/AchievementView";
import StatisticView from "./components/StatisticView";

import { Achievement, Stat, Info } from "./interfaces";
import "./App.css";

function App() {
	const isInitialized = useRef(false);
	const [achievements, setAchievements] = useState<Achievement[]>([]);
	const [icons, setIcons] = useState<{ [key: string]: string }>({});
	const [stats, setStats] = useState<Stat[]>([]);
	const [status, setStatus] = useState<[string, boolean][]>([]);
	const [info, setInfo] = useState<Info>({
		app_id: 0,
		app_name: "",
		user_id: 0,
		user_name: "",
	});
	const [databaseReady, setDatabaseReady] = useState<boolean>(false);

	const [view, setView] = useState<"a" | "s">("a");

	useEffect(() => {
		if (isInitialized.current) {
			return;
		}
		isInitialized.current = true;
		updateStatus("Loading database.");
		invoke("cmd_request_data").then((response) => {
			invoke("cmd_populate_data", { apps: response }).then(() => {
				updateStatus("Database ready.");
				setDatabaseReady(true);
			});
		});
	}, []);

	function updateStatus(input: string | string[]) {
		if (typeof input === "string") {
			setStatus((prev) => [
				[input, true],
				...prev.map(([message, _]) => [message, false] as [string, boolean]),
			]);
			// setStatus((prevStatus) => [input, ...prevStatus]);
		} else if (Array.isArray(input)) {
			setStatus((prev) => [
				...input.map((message) => [message, true] as [string, boolean]),
				...prev.map(([message, _]) => [message, false] as [string, boolean]),
			]);
			// setStatus((prevStatus) => [...input, ...prevStatus]);
		}
	}

	function selectView(view: string) {
		if (view == "a" || view == "s") {
			setView(view);
		}
	}

	function handleAppSelection(newID: number, newName: string) {
		if (newID > 0) {
			invoke("cmd_start_client", { appid: newID }).then((response) => {
				if (response) {
					updateStatus("Starting client.");
					LoadAchievements(false);
					LoadAchievementIcons(newID);
					LoadStatistics(newID, false);
					UpdateStatusInfo(newID, newName);
				} else {
					updateStatus(
						"Error loading client. Steam must be running and you must own the game selected.",
					);
				}
			});
		}
	}

	function LoadAchievements(refresh: boolean) {
		invoke("cmd_load_achievements").then((response) => {
			const data = response as Achievement[];
			setAchievements(data);
			refresh ? updateStatus("Achievements refreshed") : null;
		});
	}

	function LoadAchievementIcons(newID: number) {
		invoke("cmd_load_achievement_icons", { appid: newID }).then((response) => {
			const data = response as { [key: string]: string };
			setIcons(data);
		});
	}

	function LoadStatistics(newID: number, refresh: boolean) {
		invoke("cmd_load_statistics", { appid: newID }).then((response) => {
			const data = response as Stat[];
			setStats(data);
			refresh ? updateStatus("Statistics refreshed") : null;
		});
	}

	function UpdateStatusInfo(appid: number, appname: string) {
		invoke("cmd_retrieve_user").then((response) => {
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
			});
		});
	}

	return (
		<>
			<div className="sidebar">
				<Search
					onAppSelection={handleAppSelection}
					databaseReady={databaseReady}
				/>
				<Tabs handleSelectView={selectView} />
				<Status message={status} info={info} />
			</div>
			<div className="main">
				{info.app_id == 0 ? (
					<div className="instructions">
						Search for a game and select to get started
					</div>
				) : null}
				{view == "a" && info.app_id != 0 ? (
					<AchievementView
						achievements={achievements}
						icons={icons}
						updateStatus={updateStatus}
						loadAchievements={LoadAchievements}
					/>
				) : null}
				{view == "s" && info.app_id != 0 ? (
					<StatisticView
						stats={stats}
						updateStatus={updateStatus}
						loadStatistics={(refresh: boolean) => LoadStatistics(info.app_id, refresh)}
					/>
				) : null}
			</div>
		</>
	);
}

export default App;
