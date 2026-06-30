import * as $ from "svelte/internal/client";
var root_1 = $.from_svg(`<a><text>Hello</text></a>`);
var root = $.from_svg(`<svg><!></svg>`);
export default function App($$anchor) {
	var svg = root();
	{
		const s = ($$anchor) => {
			var a = root_1();
			$.append($$anchor, a);
		};
		var node = $.child(svg);
		s(node);
		$.reset(svg);
	}
	$.append($$anchor, svg);
}
