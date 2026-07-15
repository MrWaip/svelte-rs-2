import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let condition = $.prop($$props, "condition", 8);
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				$.effect(() => {
					$.document.title = "woo";
				});
			};
			$.if(node, ($$render) => {
				if (condition()) $$render(consequent);
			});
		}
		$.append($$anchor, fragment);
	});
}
