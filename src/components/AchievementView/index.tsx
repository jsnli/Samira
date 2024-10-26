import { useState, useEffect, useRef } from "react";
import "./index.css";
import { invoke } from "@tauri-apps/api/tauri";
import { Achievement } from "../../interfaces";

interface AchievementViewProps {
  achievements: Achievement[];
}

function AchievementView({ achievements }: AchievementViewProps) {
  const [items, setItems] = useState<Achievement[]>([]);
	const filterRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const achievementsClone = structuredClone(achievements);
    setItems(achievementsClone);
  }, [achievements]);

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
			console.log('Stored');
		})
  }

  return (
    <div className="achievement-view">
			<div className="nav">
				<input
					className="filter"
					type="text"
					ref={filterRef}
        	placeholder="Search by name"
				/>
				<button className="apply" onClick={apply}>Apply Changes</button>
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
                <span className="name">
                	{item.name}
                </span>
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
