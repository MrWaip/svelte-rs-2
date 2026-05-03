import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span>visible</span>`);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if ($$props.cond) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
