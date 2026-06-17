import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span> <button>x</button>`, 1);
export default function App($$anchor) {
	let arr = $.mutable_source([{ prop: "foo" }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(arr), $.index, ($$anchor, o, $$index) => {
		var fragment_1 = root_1();
		var span = $.first_child(fragment_1);
		var text = $.child(span, true);
		$.reset(span);
		var button = $.sibling(span, 2);
		$.template_effect(() => $.set_text(text, ($.get(arr)[$$index], $.untrack(() => $.get(arr)[$$index].prop))));
		$.event("click", button, () => ($.get(arr)[$$index] = {
			...$.get(arr)[$$index],
			prop: "bar"
		}, $.invalidate_inner_signals(() => $.get(arr))));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
