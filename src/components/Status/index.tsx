import "./index.css";

interface StatusProps {
	message: string;
}

function Status({ message }: StatusProps) {
	return (
		<div className="status">
			<span>Status:</span>
			<div className="status-box">{message}</div>
		</div>
	);
}

export default Status;
