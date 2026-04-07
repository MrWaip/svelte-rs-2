import * as $ from "svelte/internal/client";
var root_1 = $.from_svg(`<title>Chart</title>`);
var root = $.from_svg(`<svg><!></svg>`);
export default function App($$anchor) {
	let shown = true;
	var svg = root();
	var node = $.child(svg);
	{
		var consequent = ($$anchor) => {
			var title = root_1();
			$.append($$anchor, title);
		};
		$.if(node, ($$render) => {
			if (shown) $$render(consequent);
		});
	}
	$.reset(svg);
	$.append($$anchor, svg);
}
