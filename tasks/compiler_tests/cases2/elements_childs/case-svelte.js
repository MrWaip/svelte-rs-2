import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
var root_1 = $.from_html(`<div>text only</div> <div></div> <div></div> <div><div>more nested</div> <div>more nested</div> <div>more nested</div></div> <div><!></div> <div></div>`, 1);
export default function App($$anchor) {
	var fragment = root_1();
	var div = $.sibling($.first_child(fragment), 2);
	div.textContent = interpolation;
	var div_1 = $.sibling(div, 2);
	div_1.textContent = `concatenated + ${interpolation ?? ""} + concatenated`;
	var div_2 = $.sibling(div_1, 4);
	var node = $.child(div_2);
	{
		var consequent = ($$anchor) => {
			var div_3 = root();
			$.append($$anchor, div_3);
		};
		var consequent_1 = ($$anchor) => {
			var div_4 = root();
			$.append($$anchor, div_4);
		};
		var alternate = ($$anchor) => {
			var div_5 = root();
			$.append($$anchor, div_5);
		};
		$.if(node, ($$render) => {
			if (1 !== 1) $$render(consequent);
			else if (2 === 2) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		});
	}
	$.reset(div_2);
	$.next(2);
	$.append($$anchor, fragment);
}
