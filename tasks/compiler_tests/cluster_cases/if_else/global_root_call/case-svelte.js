import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("eee");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if ($.untrack(() => Math.max(1, 2) > 1)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
