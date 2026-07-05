import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = ["a", "b"];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 3, () => items, (item) => item, ($$anchor, item, idx) => {
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(item) ?? ""} ${$.get(idx) ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
