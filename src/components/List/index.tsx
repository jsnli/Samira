import { useState, useEffect } from "react";
import "./index.css";

interface Achievement {
	api_name: string;
	name: string;
	desc: string;
	status: boolean;
}

interface ListProps {
	achievements: Achievement[];
}

function List({ achievements }: ListProps) {
	const [items, setItems] = useState<Achievement[]>([]);

	useEffect(() => {
		const achievementsClone = structuredClone(achievements);
		setItems(achievementsClone);
	}, [achievements]);

	function handleCheckbox( index: number ) {
		const newItems = [...items];
		newItems[index].status = !(items[index].status);
		setItems(newItems);
	}

	function apply() {
		console.log("apply");
	}

	return (
		<>
			<button onClick={apply}>Apply Changes</button>
			<ul className="list">
				{items.map((item, index) => (
					<li key={index}>
						<label>
							<input
								type="checkbox"
								checked={item.status}
								onChange={() => handleCheckbox(index)}
							/>
							<div>
								<span className="name">
									{index} - {item.name} - {item.status}
								</span>
								<span className="desc">{item.desc}</span>
							</div>
						</label>
					</li>
				))}
			</ul>
		</>
	);
}

export default List;
