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
		var alternate = ($$anchor) => {
			var text_1 = $.text("rrr");
			$.append($$anchor, text_1);
		};
		$.if(node, ($$render) => {
			if ($.untrack(() => "Eva".startsWith("E"))) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
