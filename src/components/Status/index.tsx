import "./index.css";
import { Info } from "../../interfaces";

interface StatusProps {
  message: [string, boolean][];
  info: Info;
}

function Status({ message, info }: StatusProps) {

  return (
    <div className="status">
      {info.app_id > 0 ? (
        <img
          src={
            "https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/" +
            info.app_id +
            "/header.jpg"
          }
        />
      ) : null}
      <span className="header">Information:</span>
      <div className="info">
        <label>Game:</label>
        <span>{info.app_name}</span>
      </div>
      <div className="info">
        <label>AppID:</label>
        <span>{info.app_id !== 0 ? info.app_id : null}</span>
      </div>
      <div className="info">
        <label>Name:</label>
        <span>{info.user_name}</span>
      </div>
      <div className="info">
        <label>SteamID:</label>
        <span>{info.user_id !== 0 ? info.user_id : null}</span>
      </div>
      <div className="log">
        <span>Activity:</span>
        <ul>
          {message.map((element, index) => (
            <li className={element[1] ? "recent" : undefined} key={index}>{element[0]}</li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default Status;
