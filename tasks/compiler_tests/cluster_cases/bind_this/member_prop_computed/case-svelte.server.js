import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let container = $.fallback($$props["container"], () => ({}), true);
		let paths = $.fallback($$props["paths"], () => ["a"], true);
		$$renderer.push(`<div></div>`);
		$.bind_props($$props, {
			container,
			paths
		});
	});
}
