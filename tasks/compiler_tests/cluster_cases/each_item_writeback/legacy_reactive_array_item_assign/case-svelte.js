import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let arr = $.mutable_source([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(arr), $.index, ($$anchor, o, $$index) => {
		var button = root();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(arr)[$$index]));
		$.event("click", button, () => {
			$.get(arr)[$$index] = $.get(arr)[$$index] * 2, $.invalidate_inner_signals(() => $.get(arr));
		});
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
