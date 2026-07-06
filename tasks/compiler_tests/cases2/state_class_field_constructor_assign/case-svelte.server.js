import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			count = 0;
			constructor(initial) {
				this.count = initial;
			}
		}
		let c = new Counter(10);
		$$renderer.push(`<p>${$.escape(c.count)}</p>`);
	});
}
