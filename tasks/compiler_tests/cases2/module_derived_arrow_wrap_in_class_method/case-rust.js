import * as $ from "svelte/internal/client";
export class Store {
	viewOf(id) {
		const item = $.derived(() => id + 1);
		return $.get(item);
	}
}
