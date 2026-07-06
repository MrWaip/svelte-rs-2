import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["checked"]);
	let checked = $.fallback($$props["checked"], false);
	$$renderer.push(`<input${$.attributes({
		checked,
		type: "checkbox",
		...$$restProps
	}, void 0, void 0, void 0, 4)}/>`);
	$.bind_props($$props, { checked });
}
