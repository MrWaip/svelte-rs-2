import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	let a = $$props["a"];
	let b = $$props["b"];
	$: a, $$sanitized_props, b;
	$.bind_props($$props, {
		a,
		b
	});
}
