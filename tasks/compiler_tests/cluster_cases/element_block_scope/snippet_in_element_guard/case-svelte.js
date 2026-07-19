import * as $ from "svelte/internal/client";
var root = $.from_html(`<b>hi</b>`);
var root_1 = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	var div = root_1();
	{
		const foo = ($$anchor) => {
			var b = root();
			$.append($$anchor, b);
		};
		var node = $.child(div);
		foo(node);
		$.reset(div);
	}
	$.append($$anchor, div);
}
