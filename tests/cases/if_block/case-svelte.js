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
			var node_1 = $.sibling(text);
			{
				var consequent = ($$anchor) => {
					var text_1 = $.text();
					$.template_effect(() => $.set_text(text_1, `120 = ${id ?? ""}`));
					$.append($$anchor, text_1);
				};
				$.if(node_1, ($$render) => {
					if (id) $$render(consequent);
				});
			}
			$.reset(div);
			$.template_effect(() => $.set_text(text, `${id ?? ""} `));
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent_1);
		});
	}
	$.append($$anchor, fragment);
}
