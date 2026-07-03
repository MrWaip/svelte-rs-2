import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>Active</p>`);
export default function App($$anchor) {
	let count = 0;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		var d = $.derived(() => $.eager(() => count));
		$.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
