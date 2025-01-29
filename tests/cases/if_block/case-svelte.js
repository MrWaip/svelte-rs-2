import * as $ from "svelte/internal/client";
var root_1 = $.template(`<div name="123"> </div> <span>text</span>`, 1);
export default function App($$anchor) {
	let id = undefined;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root_1();
			var div = $.first_child(fragment_1);
			var text = $.child(div, true);
			$.reset(div);
			$.next(2);
			$.template_effect(() => $.set_text(text, id));
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
