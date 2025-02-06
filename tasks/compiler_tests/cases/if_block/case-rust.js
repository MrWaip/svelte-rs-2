import * as $ from "svelte/internal/client";
var root_1 = $.template(`<div name="123"> <!></div>`);
export default function App($$anchor) {
	let id = undefined;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent_1 = ($$anchor) => {
			var div = root_1();
			var text = $.child(div);
			text.textContent = `${id ?? ""} `;
			var node_1 = $.sibling(text);
			{
				var consequent = ($$anchor) => {
					var text_1 = $.text();
					text_1.textContent = `120 = ${id ?? ""}`;
					$.append($$anchor, text_1);
				};
				$.if(node_1, ($$render) => {
					if (id) $$render(consequent);
				});
			}
			$.reset(div);
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent_1);
		});
	}
	$.append($$anchor, fragment);
}
