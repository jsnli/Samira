export interface Achievement {
	api_name: string;
	name: string;
	desc: string;
	status: boolean;
}

export interface Stat {
	api_name: string;
	value: number;
}

export interface Info {
	app_id: number;
	app_name: string;
	user_id: number;
	user_name: string;
}

export interface App {
  appid: number;
  name: string;
  last_modified: number;
  price_change_number: number;
}
