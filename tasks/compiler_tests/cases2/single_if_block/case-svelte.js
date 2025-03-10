import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("some text");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if (visible) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
