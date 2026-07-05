import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			constructor() {
				this.count = 0;
			}
		}
		let c = new Counter();
		$$renderer.push(`<p>${$.escape(c.count)}</p>`);
	});
}
