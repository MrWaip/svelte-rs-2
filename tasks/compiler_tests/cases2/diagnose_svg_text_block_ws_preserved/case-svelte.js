import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><text><!> <!></text></svg>`);
export default function App($$anchor) {
	var svg = root();
	var text = $.child(svg);
	var node = $.child(text);
	{
		var consequent = ($$anchor) => {
			var text_1 = $.text("hello");
			$.append($$anchor, text_1);
		};
		$.if(node, ($$render) => {
			if (cond) $$render(consequent);
		});
	}
	var node_1 = $.sibling(node, 2);
	{
		var consequent_1 = ($$anchor) => {
			var text_2 = $.text("world");
			$.append($$anchor, text_2);
		};
		$.if(node_1, ($$render) => {
			if (cond) $$render(consequent_1);
		});
	}
	$.reset(text);
	$.reset(svg);
	$.append($$anchor, svg);
}
