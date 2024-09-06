import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

	const [steamID, getSteamID] = useState<string>("")

	useEffect(() => {
		invoke('get_steam_id').then((response) => {
			if (typeof response === 'string') {
				getSteamID(response)
			}
		})
	}, [])
	
	return (
		<>
			<p>{steamID}</p>	
		</>
	);
}

export default App;
