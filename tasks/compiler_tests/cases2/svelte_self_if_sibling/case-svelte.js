import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>ok</p> <!>`, 1);
export default function App($$anchor) {
	let count = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root();
			var node_1 = $.sibling($.first_child(fragment_1), 2);
			App(node_1, {});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (count > 0) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
