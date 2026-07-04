import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 8);
	function build(v) {
		return { value: v };
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(() => build(value()));
		$.add_svelte_meta(() => Child($$anchor, $.spread_props(() => $.get($0))), "component", App, 7, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
