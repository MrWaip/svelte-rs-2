import * as $ from "svelte/internal/client";
var root_1 = $.template(`<div></div>`);
var root = $.template(`<div>text only</div> <div></div> <div></div> <div><div>more nested</div> <div>more nested</div> <div>more nested</div></div> <div><!></div> <div></div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var div = $.sibling($.first_child(fragment), 2);
	div.textContent = interpolation;
	var div_1 = $.sibling(div, 2);
	div_1.textContent = `concatenated + ${interpolation ?? ""} + concatenated`;
	var div_2 = $.sibling(div_1, 4);
	var node = $.child(div_2);
	{
		var consequent = ($$anchor) => {
			var div_3 = root_1();
			$.append($$anchor, div_3);
		};
		$.if(node, ($$render) => {
			if (1 !== 1) $$render(consequent);
		});
	}
	$.next(2);
	$.append($$anchor, fragment);
}
