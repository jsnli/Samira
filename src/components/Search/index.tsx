import { ChangeEvent, useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./index.css";
import { App } from "../../interfaces";

interface SearchProps {
  onAppSelection: (newID: number, newName: string) => void;
  setStatus: (message: string) => void;
  databaseReady: boolean;
}

function Search({ onAppSelection, setStatus, databaseReady }: SearchProps) {
  const [query, setQuery] = useState("");
  const [applist, setApplist] = useState<App[]>([]);
  const [active, setActive] = useState<boolean>(false);

  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    let debounce = setTimeout(() => {
      search(query);
    }, 1000);

    return () => {
      clearTimeout(debounce);
    };
  }, [query]);

  function handleFocus() {
    setActive(true);
  }

  function handleBlur() {
    setActive(false);
  }

  function search(value: string) {
    if (!value.length) return;
    invoke("cmd_query_name", { name: value }).then((response) => {
      setApplist(response as App[]);
    });
  }

  function handleChange(event: ChangeEvent<HTMLInputElement>) {
    setQuery(event.target.value);
  }

  function handleItemClick(app: App) {
    if (inputRef.current) {
      inputRef.current.value = app.name;
    }
    onAppSelection(app.appid, app.name);
    setActive(false);
  }

  function handleAppIDLaunch(id: number) {
    invoke("cmd_query_id", { appid: Number(query) }).then((response) => {
      const app = response as App;
      if (app.appid > 0) {
				handleItemClick(app);
      } else {
        setStatus(`No game found for App ID: ${id}`);
      }
    });
  }

  return (
    <div
      className="search"
      tabIndex={0}
      onFocus={handleFocus}
      onBlur={handleBlur}
    >
      <input
        type="text"
        ref={inputRef}
        placeholder="Search by name"
        className="search-input"
        onChange={handleChange}
        disabled={!databaseReady}
      />
      <ul
        className={`search-dropdown ${active ? "search-active" : "search-hidden"}`}
      >
        {!isNaN(Number(query)) && query.length > 0 ? (
          <li
            className="search-item"
            onClick={() => handleAppIDLaunch(Number(query))}
          >
            Launch AppId - {query}
          </li>
        ) : null}
        {applist.map((app, index) => (
          <li
            className="search-item"
            key={index}
            onClick={() => handleItemClick(app)}
          >
            {app.name} - {app.appid}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Search;
