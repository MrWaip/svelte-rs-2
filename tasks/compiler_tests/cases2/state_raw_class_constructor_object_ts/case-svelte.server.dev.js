import * as $ from "svelte/internal/server";
export class Store {
	value;
	constructor() {
		this.value = { type: "idle" };
	}
}
