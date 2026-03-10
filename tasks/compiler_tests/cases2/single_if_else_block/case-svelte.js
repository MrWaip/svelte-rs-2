import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("if text");
			$.append($$anchor, text);
		};
		var consequent_1 = ($$anchor) => {
			var text_1 = $.text("else if text");
			$.append($$anchor, text_1);
		};
		var alternate = ($$anchor) => {
			var text_2 = $.text("else text");
			$.append($$anchor, text_2);
		};
		$.if(node, ($$render) => {
			if (visible) $$render(consequent);
			else if (false) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}
