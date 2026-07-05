import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			x = 0;
		}
		const c = new Counter();
		$$renderer.push(`<p>${$.escape(c.x)}</p>`);
	});
}
