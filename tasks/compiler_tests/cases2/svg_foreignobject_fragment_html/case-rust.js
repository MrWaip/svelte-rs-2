import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div>fallback html</div>`);
var root = $.from_svg(`<foreignObject><!></foreignObject>`);
export default function App($$anchor) {
	let shown = true;
	var foreignObject = root();
	var node = $.child(foreignObject);
	{
		var consequent = ($$anchor) => {
			var div = root_1();
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (shown) $$render(consequent);
		});
	}
	$.reset(foreignObject);
	$.append($$anchor, foreignObject);
}
