import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy(["a", "b"]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		const count = $.derived(() => items.length);
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
