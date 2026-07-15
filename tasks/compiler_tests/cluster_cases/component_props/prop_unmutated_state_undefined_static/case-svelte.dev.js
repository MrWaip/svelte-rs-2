App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = void 0;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { value }), "component", App, 6, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
