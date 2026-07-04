import * as $ from "svelte/internal/client";
const DEFAULTS = { mode: "idle" };
export class Store {
	#current = $.tag($.state($.proxy(DEFAULTS)), "Store.current");
	get current() {
		return $.get(this.#current);
	}
	set current(value) {
		$.set(this.#current, value, true);
	}
}
