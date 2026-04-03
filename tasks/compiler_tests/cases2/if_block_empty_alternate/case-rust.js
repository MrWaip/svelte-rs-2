import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>hello</p>`);
export default function App($$anchor) {
	let show = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		var alternate = ($$anchor) => {};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}
