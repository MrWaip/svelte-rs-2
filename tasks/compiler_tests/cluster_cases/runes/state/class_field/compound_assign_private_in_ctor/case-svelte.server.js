import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#a;
			#b = { val: -1 };
			#c;
			constructor() {
				this.#a ||= { val: 0 };
				this.#b &&= { val: 0 };
				this.#c ??= { val: 0 };
			}
			get a() {
				return this.#a?.val;
			}
		}
		const counter = new Counter();
		$$renderer.push(`<p>${$.escape(counter.a)}</p>`);
	});
}
