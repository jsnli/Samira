import { ChangeEvent, useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./index.css";

interface App {
  appid: number;
  name: string;
  last_modified: number;
  price_change_number: number;
}

interface SearchProps {
  onDropdownClick: (newID: number) => void;
}

function Search({ onDropdownClick }: SearchProps) {
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
		onDropdownClick(app.appid);
		setActive(false);
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
      />
      <ul
        className={`search-dropdown ${active ? "search-active" : "search-hidden"}`}
      >
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
