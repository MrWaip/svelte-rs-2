App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><a href="/x">link</a> text&nbsp;more <!></div>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.sibling($.child(div), 2);
	{
		var consequent = ($$anchor) => {
			var text = $.text("x");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 6, 4);
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
