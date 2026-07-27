import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		let count = () => $.get($$item).count;
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, count()));
		$.event("click", button, () => ($.get($$item).count = 5, $.invalidate_inner_signals(() => rows())));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
