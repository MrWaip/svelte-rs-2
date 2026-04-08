import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	let items = $.proxy(["a", "b"]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item, index) => {
		var span = root_1();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${index}: ${$.get(item) ?? ""}`));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
