import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		let doubled = $.derived(() => {
			return count * 2;
		});
	});
}
