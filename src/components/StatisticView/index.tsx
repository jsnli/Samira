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
		</div>
	)

}

export default StatisticView;
