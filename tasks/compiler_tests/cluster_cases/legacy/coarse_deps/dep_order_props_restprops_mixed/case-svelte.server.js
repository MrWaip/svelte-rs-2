import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["a"]);
	let a = $$props["a"];
	$: $$sanitized_props, a, $$restProps;
	$.bind_props($$props, { a });
}
