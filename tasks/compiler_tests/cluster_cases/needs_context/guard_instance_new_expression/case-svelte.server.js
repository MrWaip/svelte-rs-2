import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 0;
		function make() {
			return new Date();
		}
		$$renderer.push(`<button>${$.escape(n)}</button>`);
	});
}
