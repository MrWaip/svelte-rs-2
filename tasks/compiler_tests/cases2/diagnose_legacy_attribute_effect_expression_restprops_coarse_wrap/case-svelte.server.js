import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["name", "checked"]);
	$$renderer.component(($$renderer) => {
		let name = $.fallback($$props["name"], "");
		let checked = $.fallback($$props["checked"], false);
		$$renderer.push(`<input${$.attributes({
			type: "checkbox",
			checked,
			id: $$restProps.id || name,
			...$$restProps
		}, void 0, void 0, void 0, 4)}/>`);
		$.bind_props($$props, {
			name,
			checked
		});
	});
}
