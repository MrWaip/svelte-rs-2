import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			increment() {
				this.#count += 1;
			}
		}
		const counter = new Counter();
		function handleError(e) {
			console.error(e, counter);
		}
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<!---->x`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
