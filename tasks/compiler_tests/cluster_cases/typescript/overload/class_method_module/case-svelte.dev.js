import * as $ from "svelte/internal/client";
export class Store {
	log(a) {
		console.log(...$.log_if_contains_state("log", a));
	}
}
