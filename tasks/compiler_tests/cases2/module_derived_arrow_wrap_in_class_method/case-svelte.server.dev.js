import * as $ from "svelte/internal/server";
export class Store {
	viewOf(id) {
		const item = $.derived(() => id + 1);
		return item();
	}
}
