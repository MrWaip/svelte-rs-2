import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {}
		$$renderer.push(`<p>${$.escape(Counter.name)}</p>`);
		$.bind_props($$props, { Counter });
	});
}
