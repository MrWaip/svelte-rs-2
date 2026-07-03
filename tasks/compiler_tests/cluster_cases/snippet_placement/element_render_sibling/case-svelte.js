import * as $ from "svelte/internal/client";
var root = $.from_html(`<span>hi</span>`);
var root_1 = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	var div = root_1();
	{
		const t = ($$anchor) => {
			var span = root();
			$.append($$anchor, span);
		};
		var node = $.child(div);
		t(node);
		$.reset(div);
	}
	$.append($$anchor, div);
}
