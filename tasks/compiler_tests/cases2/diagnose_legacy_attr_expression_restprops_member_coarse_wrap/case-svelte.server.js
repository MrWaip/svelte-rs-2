import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["name"]);
	$$renderer.component(($$renderer) => {
		let name = $.fallback($$props["name"], "n");
		$$renderer.push(`<div${$.attr("id", $$restProps.id || name)}></div>`);
		$.bind_props($$props, { name });
	});
}
