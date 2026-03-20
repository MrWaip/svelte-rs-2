import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>Loading</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.eager($.pending)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
