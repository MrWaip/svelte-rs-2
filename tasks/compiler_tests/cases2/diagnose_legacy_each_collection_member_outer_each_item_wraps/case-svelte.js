import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 1, () => ($.get(item), $.untrack(() => $.get(item).kids)), $.index, ($$anchor, kid) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(kid)));
			$.append($$anchor, p);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
