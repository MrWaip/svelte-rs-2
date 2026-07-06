import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["variant", "size"]);
	let variant = $.fallback($$props["variant"], "filled");
	let size = $.fallback($$props["size"], "md");
	$$renderer.push(`<button${$.attributes({
		...$$restProps,
		class: `variant-${$.stringify(variant)} size-${$.stringify(size)}`
	})}>click me</button>`);
	$.bind_props($$props, {
		variant,
		size
	});
}
