App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let handler = (error) => console.error(...$.log_if_contains_state("error", error));
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { get handler() {
		return handler;
	} }), "component", App, 6, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
