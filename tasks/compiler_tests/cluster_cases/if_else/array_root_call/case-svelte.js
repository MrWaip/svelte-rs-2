import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("eee");
			$.append($$anchor, text);
		};
		var d = $.derived(() => $.untrack(() => [1, 2].includes(1)));
		$.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
