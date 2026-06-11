import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $.prop($$props, "x", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if ($$slots) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
