App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<svg><text><!> <!></text></svg>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var svg = root();
	var text = $.child(svg);
	var node = $.child(text);
	{
		var consequent = ($$anchor) => {
			var text_1 = $.text("hello");
			$.append($$anchor, text_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (cond) $$render(consequent);
		}), "if", App, 3, 2);
	}
	var node_1 = $.sibling(node, 2);
	{
		var consequent_1 = ($$anchor) => {
			var text_2 = $.text("world");
			$.append($$anchor, text_2);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if (cond) $$render(consequent_1);
		}), "if", App, 4, 2);
	}
	$.reset(text);
	$.reset(svg);
	$.append($$anchor, svg);
	return $.pop($$exports);
}
