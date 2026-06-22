import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function fade(node) {
		return {};
	}
	let show = true;
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root_1();
			$.transition(3, div, () => fade, () => ({ duration: 200 }));
			$.append($$anchor, div);
		};
		$.if(node_1, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
