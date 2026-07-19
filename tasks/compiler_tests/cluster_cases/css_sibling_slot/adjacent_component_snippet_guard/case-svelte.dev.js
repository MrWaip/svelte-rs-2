import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<v class="svelte-zsofz2"></v>`), App[$.FILENAME], [[10, 3]]);
var root_1 = $.add_locations($.from_html(`<y class="svelte-zsofz2"></y>`), App[$.FILENAME], [[8, 2]]);
var root_2 = $.add_locations($.from_html(`<span><n></n></span>`), App[$.FILENAME], [[
	20,
	3,
	[[21, 4]]
]]);
var root_3 = $.add_locations($.from_html(`<span><n></n></span>`), App[$.FILENAME], [[
	16,
	2,
	[[17, 3]]
]]);
var root_4 = $.add_locations($.from_html(`<div><x class="svelte-zsofz2"></x> <!> <z class="svelte-zsofz2"></z> <!> <m></m></div>`), App[$.FILENAME], [[
	5,
	0,
	[
		[6, 1],
		[13, 1],
		[25, 1]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_4();
	var node = $.sibling($.child(div), 2);
	{
		const foo = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var v = root();
			$.append($$anchor, v);
		});
		$.add_svelte_meta(() => Child(node, {
			foo,
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var y = root_1();
				$.append($$anchor, y);
			}),
			$$slots: {
				foo: true,
				default: true
			}
		}), "component", App, 7, 1, { componentTag: "Child" });
	}
	var node_1 = $.sibling(node, 4);
	{
		const foo = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var span = root_2();
			$.append($$anchor, span);
		});
		$.add_svelte_meta(() => Child(node_1, {
			foo,
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var span_1 = root_3();
				$.append($$anchor, span_1);
			}),
			$$slots: {
				foo: true,
				default: true
			}
		}), "component", App, 15, 1, { componentTag: "Child" });
	}
	$.next(2);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
