import './index.css';

interface StatusProps {
	appid: number; 
}

function Status({appid}: StatusProps) {
	return (
		<>
			<p>Status: {appid}</p>
		</>
	)
}

export default Status;
