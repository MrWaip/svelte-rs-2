import * as $ from "svelte/internal/client";
var root = $.template(` <div></div> <input> <div></div><!>`, 1);
export default function App($$anchor) {
	$.next();
	var fragment = root();
	var text = $.sibling($.first_child(fragment), 2);
	text.nodeValue = ` ${some_variable ?? ""} `;
	var text_1 = $.sibling(text, 2);
	text_1.nodeValue = ` text + ${name ?? ""} `;
	var node = $.sibling(text_1, 2);
	{
		var consequent = ($$anchor) => {};
		var alternate = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				var consequent_1 = ($$anchor) => {};
				var alternate_1 = ($$anchor) => {};
				$.if(node_1, ($$render) => {
					if (false) $$render(consequent_1);
else $$render(alternate_1, false);
				});
			}
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
else $$render(alternate, false);
		}, true);
	}
	$.append($$anchor, fragment);
}
