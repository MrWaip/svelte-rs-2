import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<p class="svelte-16b3ya8">hi</p>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { $$slots: { x: ($$anchor, $$slotProps) => {
		var p = root();
		$.append($$anchor, p);
	} } }), "component", App, 5, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
