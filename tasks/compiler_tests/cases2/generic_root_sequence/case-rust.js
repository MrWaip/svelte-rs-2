import * as $ from "svelte/internal/client";
var root = $.from_html(`some text <div></div> <input/> <div></div> <!>`, 1);
export default function App($$anchor) {
	$.next();
	var fragment = root();
	var text = $.sibling($.first_child(fragment), 2);
	text.nodeValue = ` ${some_variable ?? ""} `;
	var text_1 = $.sibling(text, 2);
	text_1.nodeValue = ` text + ${name ?? ""} `;
	var node = $.sibling(text_1, 3);
	{
		var consequent = ($$anchor) => {};
		var consequent_1 = ($$anchor) => {};
		var alternate = ($$anchor) => {};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
			else if (false) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}
