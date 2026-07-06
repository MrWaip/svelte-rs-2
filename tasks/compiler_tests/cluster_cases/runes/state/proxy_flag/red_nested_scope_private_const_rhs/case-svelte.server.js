import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			set count(x) {
				const local = 5;
				this.#count = local;
			}
		}
		const counter = new Counter();
		$$renderer.push(`<button>${$.escape(counter.count)}</button>`);
	});
}
