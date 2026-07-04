App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	$.set(count, 1);
	let doubled = $.tag($.derived(() => $.get(count) * 2), "doubled");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { get doubled() {
		return $.get(doubled);
	} }), "component", App, 8, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
