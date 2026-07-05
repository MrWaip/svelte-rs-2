import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			#doubled = $.derived(() => this.#count * 2);
			constructor() {
				console.log(this.#doubled());
			}
		}
		let c = new Counter();
		$$renderer.push(`<p>${$.escape(c.display)}</p>`);
	});
}
