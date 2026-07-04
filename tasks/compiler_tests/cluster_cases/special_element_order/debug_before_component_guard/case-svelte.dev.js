App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	$.template_effect(() => {
		console.log({ count: $.snapshot(count) });
		debugger;
	});
	$.add_svelte_meta(() => Child($$anchor, { count }), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
