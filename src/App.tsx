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
  const [stats, setStats] = useState<Stat[]>([]);
  const [status, setStatus] = useState<string[]>([]);
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
			setStatus(prevStatus => [input, ...prevStatus])
    } else if (Array.isArray(input)) {
   		setStatus(prevStatus => [...input, ...prevStatus]) 
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
          LoadAchievements();
          LoadStatistics(newID);
          UpdateStatusInfo(newID, newName);
        } else {
          updateStatus(
            "Error loading client. Steam must be running and you must own the game selected.",
          );
        }
      });
    }
  }

  function LoadAchievements() {
    invoke("cmd_load_achievements").then((response) => {
      const data = response as Achievement[];
      setAchievements(data);
      updateStatus("Achievements loaded.");
    });
  }

  function LoadStatistics(newID: number) {
    invoke("cmd_load_statistics", { appid: newID }).then((response) => {
      const data = response as Stat[];
      setStats(data);
			updateStatus("Statistics loaded.");
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
          updateStatus={updateStatus}
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
            updateStatus={updateStatus}
            refresh={LoadAchievements}
          />
        ) : null}
        {view == "s" && info.app_id != 0 ? (
          <StatisticView
            stats={stats}
            updateStatus={updateStatus}
            refresh={() => LoadStatistics(info.app_id)}
          />
        ) : null}
      </div>
    </>
  );
}

export default App;
