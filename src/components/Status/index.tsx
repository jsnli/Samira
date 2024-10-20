import "./index.css";
import { Info } from "../../interfaces";

interface StatusProps {
	message: string;
	info: Info;
}

function Status({ message, info }: StatusProps) {
	return (
		<div className="status">
			<span>Information:</span>	
			<table>
				<tbody>
					<tr>
						<td>Game</td>
						<td>{info.app_name}</td>
					</tr>
					<tr>
						<td>App Id</td>
						<td>{info.app_id !== 0 ? info.app_id : null}</td>
					</tr>
					<tr>
						<td>Name</td>
						<td>{info.user_name}</td>
					</tr>
					<tr>
						<td>Steam ID</td>
						<td>{info.user_id !== 0 ? info.user_id : null}</td>
					</tr>
				</tbody>
			</table>

			<span>Status:</span>
			<div className="status-box">{message}</div>
		</div>
	);
}

export default Status;
