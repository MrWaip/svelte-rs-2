import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.prop($$props, "foo", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if ($.deep_read_state(foo()), $.untrack(() => foo().bar)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
