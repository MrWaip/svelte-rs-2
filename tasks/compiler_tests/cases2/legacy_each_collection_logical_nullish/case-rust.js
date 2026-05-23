import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8, null);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items() ?? [], $.index, ($$anchor, item) => {
		var span = root_1();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
