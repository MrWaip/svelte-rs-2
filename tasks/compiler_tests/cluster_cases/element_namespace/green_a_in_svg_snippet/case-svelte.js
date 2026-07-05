import * as $ from "svelte/internal/client";
var root = $.from_svg(`<a><text>Hello</text></a>`);
var root_1 = $.from_svg(`<svg><!></svg>`);
export default function App($$anchor) {
	var svg = root_1();
	{
		const s = ($$anchor) => {
			var a = root();
			$.append($$anchor, a);
		};
		var node = $.child(svg);
		s(node);
		$.reset(svg);
	}
	$.append($$anchor, svg);
}
