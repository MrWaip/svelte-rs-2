import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	const $$restProps = $.rest_props($$sanitized_props, []);
	function wrap(props) {
		return props;
	}
	Child($$renderer, $.spread_props([wrap($$restProps)]));
}
