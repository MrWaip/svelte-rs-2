import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent_1 = ($$anchor) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				{
					var consequent = ($$anchor) => {
						$.effect(() => {
							$.document.title = "deep";
						});
					};
					$.if(node_1, ($$render) => {
						if (b()) $$render(consequent);
					});
				}
				$.append($$anchor, fragment_1);
			};
			$.if(node, ($$render) => {
				if (a()) $$render(consequent_1);
			});
		}
		$.append($$anchor, fragment);
	});
}
