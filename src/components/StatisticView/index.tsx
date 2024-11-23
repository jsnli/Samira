import { useState, useEffect } from "react";
import { Stat } from "../../interfaces";
import "./index.css";
import { invoke } from "@tauri-apps/api/core";

interface StatisticViewProps {
	stats: Stat[];
	updateStatus: (message: string | string[]) => void;
	refresh: () => void;
}

function StatisticView({ stats, updateStatus, refresh }: StatisticViewProps) {
	const [items, setItems] = useState<Stat[]>([]);

	useEffect(() => {
		const statsClone = structuredClone(stats);
		setItems(statsClone);
	}, [stats]);

	function apply() {
		const alerts: string[] = [];

		const inputs: HTMLInputElement[] = Array.from(
			document.querySelectorAll(".statistic-view ul li input"),
		);
		const values: number[] = inputs.map((input: HTMLInputElement) =>
			parseFloat(input.value),
		);

		for (let i = 0; i < items.length; i++) {
			if (items[i].value == values[i]) {
				continue;
			}
			invoke("cmd_commit_statistics", {
				name: items[i].api_name,
				value: values[i],
			}).then(() => {
				alerts.push(`${items[i].name} set to ${values[i]} `);
				console.log(`committing stats - ${items[i].api_name}`);
			});
		}

		invoke("cmd_store_stats").then(() => {
			updateStatus(alerts);
			refresh();
		});
	}

	function resetDefaultValues() {
		refresh();
		const inputs = document.querySelectorAll<HTMLInputElement>(
			".statistic-view ul li input",
		);
		for (let i = 0; i < inputs.length; i++) {
			inputs[i].value = items[i].value.toString();
		}
	}

	return (
		<div className="statistic-view">
			{items.length > 0 ? (
				<ul>
					{items.map((item, index) => (
						<li key={index}>
							<label>{item.name.length > 0 ? item.name : item.api_name}</label>
							<input
								type="number"
								min={item.min}
								max={item.max}
								defaultValue={item.value}
							/>
							<span>
								{item.min} - {item.max}
							</span>
						</li>
					))}
				</ul>
			) : (
				<span className="no-statistics">No statistics were found for this game.</span>
			)}
			<div className="nav">
				<button className="apply" onClick={apply}>
					Apply
				</button>
				<button onClick={resetDefaultValues}>Refresh</button>
			</div>
		</div>
	);
}

export default StatisticView;
