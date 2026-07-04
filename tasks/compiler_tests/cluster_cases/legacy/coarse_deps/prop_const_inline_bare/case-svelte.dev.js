import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	const k = 1;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { prop: k }), "component", App, 1, 43, { componentTag: "Child" });
	return $.pop($$exports);
}
