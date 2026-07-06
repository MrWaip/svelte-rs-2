import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.component(($$renderer) => {
		let variant = $$props["variant"];
		$$renderer.push(`<button${$.attributes({
			...$$sanitized_props,
			class: `variant-${$.stringify(variant)} ${$.stringify($$sanitized_props.class ?? "")}`
		})}>click me</button>`);
		$.bind_props($$props, { variant });
	});
}
