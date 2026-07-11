import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<meta id="m" name="title" content="woo"/>`);
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
			var alternate = ($$anchor) => {
				var meta = root();
				$.append($$anchor, meta);
			};
			$.if(node, ($$render) => {
				if (condition()) $$render(consequent);
				else $$render(alternate, -1);
			});
		}
		$.append($$anchor, fragment);
	});
}
