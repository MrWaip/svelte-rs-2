import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy([
		"a",
		"b",
		"c"
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item, i) => {
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${i}: ${$.get(item) ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
