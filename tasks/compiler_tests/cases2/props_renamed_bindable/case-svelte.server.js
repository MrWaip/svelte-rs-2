import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value: localVal = "fallback" } = $$props;
		$$renderer.push(`<p>${$.escape(localVal)}</p>`);
		$.bind_props($$props, { value: localVal });
	});
}
