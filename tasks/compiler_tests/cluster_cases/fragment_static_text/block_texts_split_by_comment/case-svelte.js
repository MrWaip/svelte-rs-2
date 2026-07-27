import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`AB`, 1);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root();
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (c) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
