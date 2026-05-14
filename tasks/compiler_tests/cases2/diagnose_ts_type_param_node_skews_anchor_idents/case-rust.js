import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	let action;
	let x = $.prop($$props, "x", 8, false);
	var div = root();
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if (x()) $$render(consequent);
		});
	}
	$.reset(div);
	$.append($$anchor, div);
}
