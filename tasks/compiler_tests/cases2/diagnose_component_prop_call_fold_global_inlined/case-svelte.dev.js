App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const SIZE = 4;
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(() => String(SIZE));
		$.add_svelte_meta(() => Child($$anchor, { get max() {
			return $.get($0);
		} }), "component", App, 6, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
