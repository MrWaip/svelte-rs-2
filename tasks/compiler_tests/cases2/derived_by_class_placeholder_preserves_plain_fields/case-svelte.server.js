import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			#total;
			get total() {
				return this.#total();
			}
			set total($$value) {
				return this.#total($$value);
			}
			width = 2;
			height = 3;
			#area = $.derived(() => this.width * this.height);
			get area() {
				return this.#area();
			}
			set area($$value) {
				return this.#area($$value);
			}
			stable = 1;
			constructor() {
				this.#total = $.derived(() => this.area + this.stable);
			}
		}
		let box = new Box();
		$$renderer.push(`<p>${$.escape(box.area)},${$.escape(box.total)},${$.escape(box.stable)}</p>`);
	});
}
