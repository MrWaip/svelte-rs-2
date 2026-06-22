import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>a</p>`);
var root_2 = $.from_html(`<p>b</p> <!>`, 1);
export default function App($$anchor) {
	let count = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		var alternate = ($$anchor) => {
			var fragment_1 = root_2();
			var node_1 = $.sibling($.first_child(fragment_1), 2);
			App(node_1, {});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (count > 0) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}
