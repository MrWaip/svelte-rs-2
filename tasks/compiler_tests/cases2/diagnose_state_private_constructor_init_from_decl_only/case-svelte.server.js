import * as $ from "svelte/internal/server";
export class A {
	#x;
	constructor() {
		this.#x = 0;
	}
	get x() {
		return this.#x;
	}
	bump() {
		this.#x = this.#x + 1;
	}
}
