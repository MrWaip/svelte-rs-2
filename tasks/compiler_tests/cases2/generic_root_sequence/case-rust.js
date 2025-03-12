import * as $ from "svelte/internal/client";
var root = $.template(`some text <div></div> <input> <div></div> <!>`, 1);
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
		var alternate = ($$anchor, $$elseif) => {
			{
				var consequent_1 = ($$anchor) => {};
				var alternate_1 = ($$anchor) => {};
				$.if($$anchor, ($$render) => {
					if (false) $$render(consequent_1);
else $$render(alternate_1, false);
				}, $$elseif);
			}
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
else $$render(alternate, false);
		});
	}
	$.append($$anchor, fragment);
}
