import "./index.css";

interface TabProps {
	handleSelectView: (view: string) => void;
}

function Tabs({ handleSelectView }:TabProps) {

	function handleTabClick(event: React.MouseEvent<HTMLButtonElement>,view: string) {

		handleSelectView(view);
		const buttons = document.querySelectorAll<HTMLButtonElement>(".tabs button");
		buttons.forEach(button => {
			button.classList.remove("active");
		})

		event.currentTarget.classList.add("active");		
	}

	return (
		<div className="tabs">
			<button className="active" onClick={(event) => handleTabClick(event, "a")}>Achievements</button>
			<button onClick={(event) => handleTabClick(event, "s")}>Statistics</button>
		</div>	
	);
}

export default Tabs;
