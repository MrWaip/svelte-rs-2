import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root_1 = $.from_html(`<span class="inside svelte-1v67kh2">inside</span>`);
var root = $.from_html(`<div class="host svelte-1v67kh2"><!></div> <span class="inside">outside</span>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var div = $.first_child(fragment);
	var node = $.child(div);
	{
		const children = ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		};
		Widget(node, {
			children,
			$$slots: { default: true }
		});
	}
	$.reset(div);
	$.next(2);
	$.append($$anchor, fragment);
}
