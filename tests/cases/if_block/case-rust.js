import * as $ from "svelte/internal/client";
var root_1 = $.template(`<div name="123"> </div>`);
export default function App($$anchor) {
	let id = undefined;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root_1();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, id));
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
