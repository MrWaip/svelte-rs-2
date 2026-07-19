import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="b svelte-v8ftti" slot="a"></div>`);
var root_1 = $.from_html(`<div class="c svelte-v8ftti" slot="b"><div class="d svelte-v8ftti"></div> <div class="e svelte-v8ftti"></div></div>`);
var root_2 = $.from_html(`<div class="a svelte-v8ftti"></div> <!> <div class="f svelte-v8ftti"></div>`, 1);
export default function App_1($$anchor) {
	let App;
	var fragment = root_2();
	var node = $.sibling($.first_child(fragment), 2);
	App(node, { $$slots: {
		a: ($$anchor, $$slotProps) => {
			var div = root();
			$.append($$anchor, div);
		},
		b: ($$anchor, $$slotProps) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		}
	} });
	$.next(2);
	$.append($$anchor, fragment);
}
