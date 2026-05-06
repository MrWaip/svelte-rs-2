import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
var root = $.from_html(`<p></p> <!>`, 1);
export default function App($$anchor) {
	const greeting = "hello";
	const items = [
		1,
		2,
		3
	];
	var fragment = root();
	var p = $.first_child(fragment);
	p.textContent = "hello";
	var node = $.sibling(p, 2);
	$.each(node, 1, () => items, $.index, ($$anchor, i) => {
		var span = root_1();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(i)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
