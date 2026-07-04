import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, item) => {
		const { x, y } = item;
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${x ?? ""} ${y ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
