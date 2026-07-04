import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Badge from "./Badge.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	function f(n) {
		return n;
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(() => ($.deep_read_state(x()), $.untrack(() => f(x()))));
		$.add_svelte_meta(() => Badge($$anchor, { get text() {
			return `a ${$.get($0) ?? ""} b`;
		} }), "component", App, 7, 0, { componentTag: "Badge" });
	}
	return $.pop($$exports);
}
