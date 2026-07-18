import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<div class="a" slot="x">a</div>`);
var root_1 = $.from_html(`<div class="b svelte-1k5tp9w" slot="y">b</div>`);
var root_2 = $.from_html(`<!> <div class="c svelte-1k5tp9w">c</div>`, 1);
export default function App($$anchor) {
	var fragment = root_2();
	var node = $.first_child(fragment);
	Child(node, { $$slots: {
		x: ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		},
		y: ($$anchor, $$slotProps) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		}
	} });
	$.next(2);
	$.append($$anchor, fragment);
}
