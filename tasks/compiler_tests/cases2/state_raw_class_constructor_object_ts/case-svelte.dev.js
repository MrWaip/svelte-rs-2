import * as $ from "svelte/internal/client";
export class Store {
	#value;
	get value() {
		return $.get(this.#value);
	}
	set value(value) {
		$.set(this.#value, value);
	}
	constructor() {
		this.#value = $.tag($.state({ type: "idle" }), "Store.value");
	}
}
