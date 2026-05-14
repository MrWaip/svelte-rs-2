import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, []);
	function wrap(props) {
		return props;
	}
	{
		let $0 = $.derived_safe_equal(() => wrap($$restProps));
		Child($$anchor, $.spread_props(() => $.get($0)));
	}
}
