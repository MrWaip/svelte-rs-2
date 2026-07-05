import * as $ from "svelte/internal/client";
export class Store {
	viewOf(id) {
		const item = $.tag($.derived(() => id + 1), "item");
		return $.get(item);
	}
}
