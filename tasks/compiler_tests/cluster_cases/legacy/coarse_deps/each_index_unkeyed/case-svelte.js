import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item, idx) => {
		const label = $.derived_safe_equal(() => (idx, $.get(item), $.untrack(() => `${idx}:${$.get(item).name}`)));
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(label)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
