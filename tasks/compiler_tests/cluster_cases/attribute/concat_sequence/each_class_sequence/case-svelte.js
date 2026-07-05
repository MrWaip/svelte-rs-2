import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item, i) => {
		var div = root();
		div.textContent = i + 1;
		$.template_effect(() => $.set_class(div, 1, `${($.get(item), $.untrack(() => $.get(item).foo ? "foo" : "")) ?? ""} ${($.get(item), $.untrack(() => $.get(item).bar ? "bar" : "")) ?? ""}`));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
