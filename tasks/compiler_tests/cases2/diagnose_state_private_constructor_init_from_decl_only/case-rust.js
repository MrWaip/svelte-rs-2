import * as $ from "svelte/internal/client";
export class A {
	#x;
	constructor() {
		this.#x = $.state(0);
	}
	get x() {
		return $.get(this.#x);
	}
	bump() {
		$.set(this.#x, $.get(this.#x) + 1);
	}
}
