export class A {
	#x;
	constructor() {
		this.#x = $state.raw(0);
	}
	get x() { return this.#x; }
	bump() { this.#x = this.#x + 1; }
}
