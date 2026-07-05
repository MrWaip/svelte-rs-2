import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let n = 0;
		$$renderer.push(`<p>${$.escape((() => {
			return n;
		})())}</p>`);
	});
}
