import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			get value() {
				return this.#count;
			}
		}
		let c = new Counter();
		$$renderer.push(`<p>${$.escape(c.value)}</p>`);
	});
}
