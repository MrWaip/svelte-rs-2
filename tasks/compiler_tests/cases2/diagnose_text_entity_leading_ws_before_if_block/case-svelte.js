import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><a href="/x">link</a> text&nbsp;more <!></div>`);
export default function App($$anchor) {
	var div = root();
	var node = $.sibling($.child(div), 2);
	{
		var consequent = ($$anchor) => {
			var text = $.text("x");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.reset(div);
	$.append($$anchor, div);
}
