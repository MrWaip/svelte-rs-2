import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div class="a" slot="x">a</div>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<div class="b svelte-1k5tp9w" slot="y">b</div>`), App[$.FILENAME], [[7, 1]]);
var root_2 = $.add_locations($.from_html(`<!> <div class="c svelte-1k5tp9w">c</div>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => Child(node, { $$slots: {
		x: ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		},
		y: ($$anchor, $$slotProps) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		}
	} }), "component", App, 5, 0, { componentTag: "Child" });
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
