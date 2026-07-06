import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let store = $.fallback($$props["store"], undefined);
		$$renderer.push(`<button>x</button>`);
		$.bind_props($$props, { store });
	});
}
