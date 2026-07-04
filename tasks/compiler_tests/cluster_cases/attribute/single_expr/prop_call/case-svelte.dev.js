import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 8);
	function fn(v) {
		return v + 1;
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(() => ($.deep_read_state(value()), $.untrack(() => fn(value()))));
		$.add_svelte_meta(() => Comp($$anchor, { get id() {
			return $.get($0);
		} }), "component", App, 6, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
