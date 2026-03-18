import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const items = [fetch("/a"), fetch("/b")];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.await(node_1, () => $.get(item), null, ($$anchor, value) => {
			var p = root_2();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(value)));
			$.append($$anchor, p);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
