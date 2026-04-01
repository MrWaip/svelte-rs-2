import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const outer = fetch("/api/list");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => outer, null, ($$anchor, items) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.await(node_1, () => $.get(items)[0], null, ($$anchor, detail) => {
			var p = root_2();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(detail)));
			$.append($$anchor, p);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
