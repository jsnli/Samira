import "./index.css";

interface Info {
	app_id: number;
	app_name: string;
	user_id: number;
	user_name: string;
}

interface StatusProps {
	message: string;
	info: Info;
}

function Status({ message, info }: StatusProps) {
	return (
		<div className="status">
			<span>Information:</span>	
			<table>
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
			</table>

			<span>Status:</span>
			<div className="status-box">{message}</div>
		</div>
	);
}

export default Status;
