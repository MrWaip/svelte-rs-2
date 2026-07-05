import * as $ from "svelte/internal/client";
var root = $.from_html(`<!-- inside if --> <span>visible</span>`, 1);
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root();
			var node_1 = $.first_child(fragment_1);
			$.next(2);
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if ($$props.cond) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
