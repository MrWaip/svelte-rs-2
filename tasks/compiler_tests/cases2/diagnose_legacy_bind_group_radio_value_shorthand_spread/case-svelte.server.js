import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["value", "group"]);
	let value = $$props["value"];
	let group = $$props["group"];
	$$renderer.push(`<input${$.attributes({
		type: "radio",
		checked: group === value,
		value,
		...$$restProps
	}, void 0, void 0, void 0, 4)}/>`);
	$.bind_props($$props, {
		value,
		group
	});
}
