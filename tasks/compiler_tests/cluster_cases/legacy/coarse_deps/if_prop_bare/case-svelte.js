import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if (foo()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
