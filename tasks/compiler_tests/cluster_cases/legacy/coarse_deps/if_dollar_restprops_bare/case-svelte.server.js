import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["x"]);
	let x = $$props["x"];
	if ($$restProps) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`a`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { x });
}
