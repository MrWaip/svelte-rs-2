import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<v class="svelte-zsofz2"></v>`);
var root_1 = $.from_html(`<y class="svelte-zsofz2"></y>`);
var root_2 = $.from_html(`<span><n></n></span>`);
var root_3 = $.from_html(`<div><x class="svelte-zsofz2"></x> <!> <z class="svelte-zsofz2"></z> <!> <m></m></div>`);
export default function App($$anchor) {
	var div = root_3();
	var node = $.sibling($.child(div), 2);
	{
		const foo = ($$anchor) => {
			var v = root();
			$.append($$anchor, v);
		};
		Child(node, {
			foo,
			children: ($$anchor, $$slotProps) => {
				var y = root_1();
				$.append($$anchor, y);
			},
			$$slots: {
				foo: true,
				default: true
			}
		});
	}
	var node_1 = $.sibling(node, 4);
	{
		const foo = ($$anchor) => {
			var span = root_2();
			$.append($$anchor, span);
		};
		Child(node_1, {
			foo,
			children: ($$anchor, $$slotProps) => {
				var span_1 = root_2();
				$.append($$anchor, span_1);
			},
			$$slots: {
				foo: true,
				default: true
			}
		});
	}
	$.next(2);
	$.reset(div);
	$.append($$anchor, div);
}
