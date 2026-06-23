import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span>hi</span>`);
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	var div = root();
	{
		const t = ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		};
		var node = $.child(div);
		t(node);
		$.reset(div);
	}
	$.append($$anchor, div);
}
