import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy(["a", "b"]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, (item) => item, ($$anchor, item) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, item));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
