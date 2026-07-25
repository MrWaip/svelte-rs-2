import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<div${$.attr("title", [() => ((y) => y)(1)])}></div>`);
	});
}
