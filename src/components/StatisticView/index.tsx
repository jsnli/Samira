import { useState, useEffect } from "react";
import { Stat } from "../../interfaces";
import "./index.css";
import { invoke } from "@tauri-apps/api/tauri";

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
    const inputs: HTMLInputElement[] = Array.from(
      document.querySelectorAll(".statistic-view ul li input"),
    );
    const values: number[] = inputs.map((input: HTMLInputElement) =>
      parseFloat(input.value),
    );

		for (let i = 0; i < items.length; i++) {
			invoke("cmd_commit_statistics", {
				name: items[i].api_name,
				value: values[i]
			}).then(() => {
				console.log(`committing stats - ${items[i].api_name}`);
			})
		}	

		invoke("cmd_store_stats").then(() => {
			console.log('Stored');
		})

  }

  return (
    <div className="statistic-view">
      <ul>
        {items.map((item, index) => (
          <li key={index}>
            <label>{item.api_name}</label>
            <input defaultValue={item.value} />
          </li>
        ))}
      </ul>
      <button className="apply" onClick={apply}>
        Apply
      </button>
    </div>
  );
}

export default StatisticView;
