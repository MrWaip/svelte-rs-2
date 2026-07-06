import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count;
			constructor() {
				this.#count = 0;
				this.total = 0;
			}
			bump() {
				this.#count++;
				this.total++;
			}
			get count() {
				return this.#count;
			}
		}
		const counter = new Counter();
		$$renderer.push(`<button>${$.escape(counter.count)} ${$.escape(counter.total)}</button>`);
	});
}
