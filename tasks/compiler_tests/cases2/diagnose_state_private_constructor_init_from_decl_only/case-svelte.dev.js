import * as $ from "svelte/internal/client";
export class A {
	#x;
	constructor() {
		this.#x = $.tag($.state(0), "A.#x");
	}
	get x() {
		return $.get(this.#x);
	}
	bump() {
		$.set(this.#x, $.get(this.#x) + 1);
	}
}
