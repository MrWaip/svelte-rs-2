import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<b> </b>`);
var root_2 = $.from_html(`<div><!></div> <div><!></div>`, 1);
export default function App($$anchor) {
	var fragment = root_2();
	var div = $.first_child(fragment);
	{
		const row = ($$anchor, n = $.noop) => {
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, n()));
			$.append($$anchor, span);
		};
		var node = $.child(div);
		row(node, () => 1);
		$.reset(div);
	}
	var div_1 = $.sibling(div, 2);
	{
		const row = ($$anchor, n = $.noop) => {
			var b = root_1();
			var text_1 = $.child(b, true);
			$.reset(b);
			$.template_effect(() => $.set_text(text_1, n()));
			$.append($$anchor, b);
		};
		var node_1 = $.child(div_1);
		row(node_1, () => 2);
		$.reset(div_1);
	}
	$.append($$anchor, fragment);
}
