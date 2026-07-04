import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<p class="svelte-1ktsoc1">hi</p>`), App[$.FILENAME], [[8, 6]]);
var root_1 = $.add_locations($.from_html(`<div class="wrap svelte-1ktsoc1"><!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	$.add_svelte_meta(() => Child(node, { $$slots: { x: ($$anchor, $$slotProps) => {
		var p = root();
		$.append($$anchor, p);
	} } }), "component", App, 6, 2, { componentTag: "Child" });
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
