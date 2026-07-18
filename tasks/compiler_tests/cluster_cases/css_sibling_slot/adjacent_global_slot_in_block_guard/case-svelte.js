import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><!> <p class="foo">foo</p></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var fragment = $.comment();
			var node_1 = $.first_child(fragment);
			$.slot(node_1, $$props, "default", {}, null);
			$.append($$anchor, fragment);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.next(2);
	$.reset(div);
	$.append($$anchor, div);
}
