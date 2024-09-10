import { ChangeEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface App {
	appid: number;
	name: string;
	last_modified: number;
	price_change_number: number;
}

function Search() {
	const [query, setQuery] = useState("");
	const [applist, setApplist] = useState<App[]>([]);

	useEffect(() => {
		let debounce = setTimeout(() => {
			search(query)
		}, 300);

		return () => {
			clearTimeout(debounce);
		}
	}, [query]);



	function search(value: string) {
		invoke("cmd_query_name", { name: value }).then((response) => {
			console.log("RESPONSE: ", response);
			setApplist(response as App[]);
		});
	}

	function handleChange(event: ChangeEvent<HTMLInputElement>) {
		setQuery(event.target.value);
	}

	return (
		<>
			<input type="text" onChange={handleChange} />
			<ul>
				{applist.map((app, index) => (
					<li key={index}>
					{app.name} - {app.appid}
					</li>
				))}
			</ul>
		</>
	);
}

export default Search;
