import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function handleClick() {}
	function getHandler() {
		return handleClick;
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(() => $.untrack(getHandler));
		$.add_svelte_meta(() => Child($$anchor, { get onclick() {
			return $.get($0);
		} }), "component", App, 10, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
