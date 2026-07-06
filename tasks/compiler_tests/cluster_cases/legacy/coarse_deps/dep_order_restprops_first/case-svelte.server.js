import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["x"]);
	let x = $$props["x"];
	$: $$restProps, x;
	$.bind_props($$props, { x });
}
