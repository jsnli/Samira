import { useState, useEffect } from "react";
import { Stat } from "../../interfaces";
import "./index.css";

interface StatisticViewProps {
	stats: Stat[];
}

function StatisticView({ stats }: StatisticViewProps) {
	const [items, setItems] = useState<Stat[]>([]);

	useEffect(() => {
		const statsClone = structuredClone(stats);
		setItems(statsClone);
	}, [stats]);

	function apply() {
		console.log("apply stats");
	}

	return (
		<div className="statistic-view">
			<ul>
				{items.map((item, index) => (
					<li key={index}>
						<label>{item.api_name}</label>
						<input value={item.value} />
					</li>
				))}
			</ul>
			<button className="apply" onClick={apply}>Apply</button>
		</div>
	)

}

export default StatisticView;
