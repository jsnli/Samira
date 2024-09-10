import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  // const [steamID, setSteamID] = useState<string>("")

  useEffect(() => {
    invoke("cmd_request_data").then((response) => {
      invoke("cmd_populate_data", { apps: response }).then(() => {
				console.log("Database Ready.");
			});
    });
  }, []);

  function handleClick() {
    invoke("cmd_query_id", { appid: 1245620 }).then((response) => {
      console.log("RESPONSE: ", response);
    });
  }

  return (
    <>
      <button onClick={handleClick}>Click</button>
    </>
  );
}

export default App;
