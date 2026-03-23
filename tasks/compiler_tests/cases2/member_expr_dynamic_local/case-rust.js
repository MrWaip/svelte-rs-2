import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let obj = null;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			p.textContent = obj.name;
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (obj) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
