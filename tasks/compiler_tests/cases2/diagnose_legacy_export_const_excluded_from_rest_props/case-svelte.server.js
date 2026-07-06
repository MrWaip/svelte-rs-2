import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, ["api", "a"]);
	let a = $$props["a"];
	const api = () => 1;
	const rest = $$restProps;
	$.bind_props($$props, {
		a,
		api
	});
}
