import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> `, 1);
export default function App($$anchor) {
	let a = 1;
	let b = 2;
	var fragment = root();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("equal");
			$.append($$anchor, text);
		};
		var consequent_1 = ($$anchor) => {
			var text_1 = $.text("one");
			$.append($$anchor, text_1);
		};
		$.if(node, ($$render) => {
			if (a === b) $$render(consequent);
			else if (a == 1) $$render(consequent_1, 1);
		});
	}
	var text_2 = $.sibling(node);
	text_2.nodeValue = " true\ntrue\ntrue";
	$.append($$anchor, fragment);
}
