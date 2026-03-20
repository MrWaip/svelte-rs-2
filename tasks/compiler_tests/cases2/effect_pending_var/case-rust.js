import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
var root_2 = $.from_html(`<p>Done</p>`);
export default function App($$anchor) {
	let count = 0;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			p.textContent = "Loading 0";
			$.append($$anchor, p);
		};
		var alternate = ($$anchor) => {
			var p_1 = root_2();
			$.append($$anchor, p_1);
		};
		$.if(node, ($$render) => {
			if ($.eager($.pending)) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}
