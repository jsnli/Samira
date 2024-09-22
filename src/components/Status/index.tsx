import './index.css';

interface StatusProps {
	appid: number; 
}

function Status({appid}: StatusProps) {
	return (
		<>
			<span>Status: {appid}</span>
		</>
	)
}

export default Status;
