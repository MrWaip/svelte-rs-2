import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <strong> </strong> <!>`, 1);
export default function App($$anchor) {
	let title = $.mutable_source("x");
	let count = $.mutable_source(0);
	function tick() {
		$.set(title, "y");
		$.set(count, 1);
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var strong = $.sibling(p, 2);
	var text_1 = $.child(strong, true);
	$.reset(strong);
	var node = $.sibling(strong, 2);
	$.element(node, () => "div", false, ($$element, $$anchor) => {
		var text_2 = $.text();
		$.template_effect(() => $.set_text(text_2, `Dyn: ${$.get(title) ?? ""}`));
		$.append($$anchor, text_2);
	});
	$.template_effect(() => {
		$.set_text(text, $.get(title));
		$.set_text(text_1, $.get(count));
	});
	$.append($$anchor, fragment);
}
