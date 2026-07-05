App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function getX() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(getX);
		$.add_svelte_meta(() => Child($$anchor, { get random() {
			return $.get($0);
		} }), "component", App, 6, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
