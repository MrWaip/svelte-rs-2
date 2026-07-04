import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { prop: $.untrack(() => Math.PI) }), "component", App, 1, 30, { componentTag: "Child" });
	return $.pop($$exports);
}
