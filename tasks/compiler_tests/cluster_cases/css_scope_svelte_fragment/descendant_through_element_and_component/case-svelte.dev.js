import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<span><p class="svelte-5iy3wu">hi</p></span>`), App[$.FILENAME], [[
	7,
	4,
	[[7, 10]]
]]);
var root_1 = $.add_locations($.from_html(`<div class="wrap svelte-5iy3wu"><!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	$.add_svelte_meta(() => Child(node, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var span = root();
			$.append($$anchor, span);
		}),
		$$slots: { default: true }
	}), "component", App, 6, 2, { componentTag: "Child" });
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
