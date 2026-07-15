App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function close() {}
	const typedClose = close;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { get handler() {
		return typedClose;
	} }), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
