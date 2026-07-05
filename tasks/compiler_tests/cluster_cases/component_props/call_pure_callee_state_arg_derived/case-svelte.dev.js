App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	$.set(count, 1);
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(() => Math.max($.get(count), 2));
		$.add_svelte_meta(() => Child($$anchor, { get random() {
			return $.get($0);
		} }), "component", App, 7, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
