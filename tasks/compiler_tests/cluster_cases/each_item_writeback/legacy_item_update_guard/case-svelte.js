import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, item, $$index) => {
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, rows()[$$index]));
		$.event("click", button, () => (rows()[$$index]++, $.invalidate_inner_signals(() => rows())));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
