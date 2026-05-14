import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<noscript></noscript>`);
export default function App($$anchor) {
	let show = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var noscript = root_1();
			$.append($$anchor, noscript);
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
