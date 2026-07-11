import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		$.each(node, 1, items, $.index, ($$anchor, i) => {
			$.deferred_template_effect(() => {
				$.document.title = $.get(i) ?? "";
			});
		});
		$.append($$anchor, fragment);
	});
}
