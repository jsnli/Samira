import "./index.css";
import { Info } from "../../interfaces";

interface StatusProps {
	message: string[];
	info: Info;
}

function Status({ message, info }: StatusProps) {
	return (
		<div className="status">
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
				<ul>		
					{message.map((element, index) => (
						<li key={index}>{element}</li>
					))}
				</ul>
			</div>
		</div>
	);
}

export default Status;
