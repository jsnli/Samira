import { useState, useEffect } from "react";
import "./index.css";
import { invoke } from "@tauri-apps/api/tauri";

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

  function handleCheckbox(index: number) {
    const newItems = [...items];
    newItems[index].status = !items[index].status;
    setItems(newItems);
  }

  function apply() {
    console.log(items);

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
    <div className="list">
      <button className="list-apply" onClick={apply}>Apply Changes</button>
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

export default List;
