import * as $ from "svelte/internal/client";
var root = $.from_svg(`<title>Chart</title>`);
var root_1 = $.from_svg(`<svg><!></svg>`);
export default function App($$anchor) {
	let shown = true;
	var svg = root_1();
	var node = $.child(svg);
	{
		var consequent = ($$anchor) => {
			var title = root();
			$.append($$anchor, title);
		};
		$.if(node, ($$render) => {
			if (shown) $$render(consequent);
		});
	}
	$.reset(svg);
	$.append($$anchor, svg);
}
