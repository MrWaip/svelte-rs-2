import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1);
export default function App($$anchor) {
	let count = 1;
	let color = "red";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root();
			var node_1 = $.first_child(fragment_1);
			{
				$.css_props(node_1, () => ({ "--my-color": color }));
				App(node_1.lastChild, {});
				$.reset(node_1);
			}
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (count > 0) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
