import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, item) => {
		const a = item, b = item * 2;
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${a ?? ""} ${b}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
