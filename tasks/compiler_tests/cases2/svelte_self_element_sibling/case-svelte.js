import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><p>ok</p><!></div>`);
export default function App($$anchor) {
	let count = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			var node_1 = $.sibling($.child(div));
			App(node_1, {});
			$.reset(div);
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (count > 0) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
