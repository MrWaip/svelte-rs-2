import "svelte/internal/flags/legacy";
App_1[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="b svelte-1u1mcs6" slot="a"></div>`), App_1[$.FILENAME], [[20, 1]]);
var root_1 = $.add_locations($.from_html(`<div class="c" slot="b"><div class="d svelte-1u1mcs6"></div> <div class="e svelte-1u1mcs6"></div></div>`), App_1[$.FILENAME], [[
	22,
	1,
	[[23, 2], [24, 2]]
]]);
var root_2 = $.add_locations($.from_html(`<div class="f svelte-1u1mcs6" slot="c"></div>`), App_1[$.FILENAME], [[27, 1]]);
var root_3 = $.add_locations($.from_html(`<div class="a svelte-1u1mcs6"></div> <!> <div class="g svelte-1u1mcs6"></div>`, 1), App_1[$.FILENAME], [[18, 0], [30, 0]]);
export default function App_1($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App_1);
	let App;
	var $$exports = { ...$.legacy_api() };
	var fragment = root_3();
	var node = $.sibling($.first_child(fragment), 2);
	$.add_svelte_meta(() => App(node, { $$slots: {
		a: ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		},
		b: ($$anchor, $$slotProps) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		},
		c: ($$anchor, $$slotProps) => {
			var div_2 = root_2();
			$.append($$anchor, div_2);
		}
	} }), "component", App_1, 19, 0, { componentTag: "App" });
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
