import * as $ from "svelte/internal/client";
var root = $.from_html(`<b>hi</b>`);
var root_1 = $.from_html(`<div><span></span></div>`);
export default function App($$anchor) {
	var div = root_1();
	var span = $.child(div);
	{
		const foo = ($$anchor) => {
			var b = root();
			$.append($$anchor, b);
		};
	}
	$.reset(div);
	$.append($$anchor, div);
}
