import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<p>err</p>`);
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let error = $.state(void 0);
	var fragment = root();
	var node = $.first_child(fragment);
	$.boundary(node, { onerror: (e) => $.set(error, e, true) }, ($$anchor) => {
		$.next();
		var text = $.text("x");
		$.append($$anchor, text);
	});
	var node_1 = $.sibling(node, 2);
	{
		var consequent = ($$anchor) => {
			var p = root_2();
			$.append($$anchor, p);
		};
		$.if(node_1, ($$render) => {
			if ($.get(error)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
