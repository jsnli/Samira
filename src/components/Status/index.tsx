import './index.css';


interface StatusProps {
	message: string;
}

function Status({message}: StatusProps) {
	return (
		<>
			<p>Status: {message}</p>
		</>
	)
}

export default Status;
