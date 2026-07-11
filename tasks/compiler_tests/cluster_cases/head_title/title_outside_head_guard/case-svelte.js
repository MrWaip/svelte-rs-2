import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<title>nothead</title>`);
export default function App($$anchor, $$props) {
	let condition = $.prop($$props, "condition", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var title = root();
			$.append($$anchor, title);
		};
		$.if(node, ($$render) => {
			if (condition()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
