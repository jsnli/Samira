import { ChangeEvent, useState, useEffect, useRef } from "react";
import "./index.css";
import { invoke } from "@tauri-apps/api/tauri";
import { Achievement } from "../../interfaces";

interface AchievementViewProps {
	achievements: Achievement[];
	refresh: () => void;
}

function AchievementView({ achievements, refresh }: AchievementViewProps) {
	const [items, setItems] = useState<Achievement[]>([]);
	const [filter, setFilter] = useState("");
	const filterRef = useRef<HTMLInputElement>(null);

	useEffect(() => {
		const achievementsClone = structuredClone(achievements);
		setItems(achievementsClone);
	}, [achievements]);

	useEffect(() => {
		let debounce = setTimeout(() => {
			filterItems(filter);	
		}, 1000);

		return () => {
			clearTimeout(debounce);
		};
	}, [filter]);

	function handleFilterInput(event: ChangeEvent<HTMLInputElement>) {
		setFilter(event.target.value);
	}

	function filterItems(search: string) {
		if (search.length < 1) {
			refresh()
			return
		}
		
		const filteredItems = achievements.filter(function(item) {
			if (item.name.toLowerCase().includes(search)) {
				return true
			}

			if (item.desc.toLowerCase().includes(search)) {
				return true
			}

			return false;
		});

		setItems(filteredItems);
	}

	function handleCheckbox(index: number) {
		const newItems = [...items];
		newItems[index].status = !items[index].status;
		setItems(newItems);
	}

	function apply() {
		for (let i = 0; i < items.length; i++) {
			if (items[i].status != achievements[i].status) {
				invoke("cmd_commit_achievement", {
					name: items[i].api_name,
					unlocked: items[i].status,
				}).then(() => {
					console.log(`${items[i].name} commited as - ${items[i].status}`);
				});
			}
		}

		invoke("cmd_store_stats").then(() => {
			console.log("Stored");
		});
	}

	function selectAll(newStatus: boolean) {
		const itemsClone = structuredClone(items);
		for (let i = 0; i < itemsClone.length; i++) {
			itemsClone[i].status = newStatus;
		}

		setItems(itemsClone);
	}

	return (
		<div className="achievement-view">
			<div className="nav">
				<input
					className="filter"
					type="text"
					ref={filterRef}
					placeholder="Filter by name or description"
					onChange={handleFilterInput}
				/>
				<button onClick={refresh}>Refresh</button>
				<button className="apply" onClick={apply}>
					Apply Changes
				</button>
			</div>
			<div className="subnav">
				<button onClick={() => selectAll(true)}>Select All</button>
				<button onClick={() => selectAll(false)}>Deselect All</button>
			</div>
			<ul>
				{items.map((item, index) => (
					<li key={index}>
						<label>
							<input
								type="checkbox"
								checked={item.status}
								onChange={() => handleCheckbox(index)}
							/>
							<div>
								<span className="name">{item.name}</span>
								<span className="desc">{item.desc}</span>
							</div>
						</label>
					</li>
				))}
			</ul>
		</div>
	);
}

export default AchievementView;
