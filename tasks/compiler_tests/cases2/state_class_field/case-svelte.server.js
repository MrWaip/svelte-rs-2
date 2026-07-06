import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			count = 0;
			increment() {
				this.count += 1;
			}
		}
		let c = new Counter();
		$$renderer.push(`<p>${$.escape(c.count)}</p>`);
	});
}
