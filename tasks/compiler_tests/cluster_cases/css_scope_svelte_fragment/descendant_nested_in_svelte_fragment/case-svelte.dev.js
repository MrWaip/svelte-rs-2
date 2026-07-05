import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div class="mid svelte-1pc7tr6"><p class="svelte-1pc7tr6">hi</p></div>`), App[$.FILENAME], [[
	8,
	6,
	[[8, 23]]
]]);
var root_1 = $.add_locations($.from_html(`<div class="wrap svelte-1pc7tr6"><!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	$.add_svelte_meta(() => Child(node, { $$slots: { x: ($$anchor, $$slotProps) => {
		var div_1 = root();
		$.append($$anchor, div_1);
	} } }), "component", App, 6, 2, { componentTag: "Child" });
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
