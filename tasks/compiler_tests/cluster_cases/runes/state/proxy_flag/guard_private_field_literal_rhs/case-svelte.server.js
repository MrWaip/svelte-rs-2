import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			get count() {
				return this.#count;
			}
			reset() {
				this.#count = 5;
			}
		}
		const counter = new Counter();
		$$renderer.push(`<button>${$.escape(counter.count)}</button>`);
	});
}
