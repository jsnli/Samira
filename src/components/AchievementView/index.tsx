import { ChangeEvent, useState, useEffect, useRef } from "react";
import "./index.css";
import { invoke } from "@tauri-apps/api/core";
import { Achievement } from "../../interfaces";

interface AchievementViewProps {
  achievements: Achievement[];
  updateStatus: (message: string | string[]) => void;
  refresh: () => void;
}

function AchievementView({
  achievements,
  updateStatus,
  refresh,
}: AchievementViewProps) {
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
    const filteredItems = achievements.filter(function (item) {
      if (search.length < 1) {
        return true;
      }
      if (item.name.toLowerCase().includes(search.toLowerCase())) {
        return true;
      }

      if (item.desc.toLowerCase().includes(search.toLowerCase())) {
        return true;
      }

      return false;
    });

    setItems(filteredItems);
  }

  function handleCheckbox(index: number) {
    const newItems = structuredClone(items);
    newItems[index].status = !items[index].status;
    setItems(newItems);
  }

  function apply() {
    const alerts: string[] = [];
    console.log(items, achievements);
    for (let i = 0; i < items.length; i++) {
      if (items[i].status != achievements[i].status) {
        invoke("cmd_commit_achievement", {
          name: items[i].api_name,
          unlocked: items[i].status,
        }).then(() => {
          alerts.push(
            `${items[i].name} ${items[i].status ? "unlocked" : "locked"}`,
          );
        });
      }
    }

    invoke("cmd_store_stats").then(() => {
      updateStatus(alerts);
    });
  }

  function selectAll(newStatus: boolean) {
    const itemsClone = structuredClone(items);
    for (let i = 0; i < itemsClone.length; i++) {
      itemsClone[i].status = newStatus;
    }

    setItems(itemsClone);
  }

  function selectLocked(locked: boolean) {
    const lockedItems = achievements.filter(function (item) {
      if (item.status === locked) {
        return true;
      }
    });

    setItems(lockedItems);
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
        <button onClick={() => selectLocked(false)}>Locked Only</button>
        <button onClick={() => selectLocked(true)}>Unlocked Only</button>
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
