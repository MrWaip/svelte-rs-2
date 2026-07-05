import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	const $$restProps = $.legacy_rest_props($$sanitized_props, []);
	$.push($$props, false, App);
	function wrap(props) {
		return props;
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(() => wrap($$restProps));
		$.add_svelte_meta(() => Child($$anchor, $.spread_props(() => $.get($0))), "component", App, 9, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
