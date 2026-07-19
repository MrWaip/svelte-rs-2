import "svelte/internal/flags/legacy";
App_1[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="b svelte-v8ftti" slot="a"></div>`), App_1[$.FILENAME], [[16, 1]]);
var root_1 = $.add_locations($.from_html(`<div class="c svelte-v8ftti" slot="b"><div class="d svelte-v8ftti"></div> <div class="e svelte-v8ftti"></div></div>`), App_1[$.FILENAME], [[
	18,
	1,
	[[19, 2], [20, 2]]
]]);
var root_2 = $.add_locations($.from_html(`<div class="a svelte-v8ftti"></div> <!> <div class="f svelte-v8ftti"></div>`, 1), App_1[$.FILENAME], [[14, 0], [24, 0]]);
export default function App_1($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App_1);
	let App;
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var node = $.sibling($.first_child(fragment), 2);
	$.add_svelte_meta(() => App(node, { $$slots: {
		a: ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		},
		b: ($$anchor, $$slotProps) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		}
	} }), "component", App_1, 15, 0, { componentTag: "App" });
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
