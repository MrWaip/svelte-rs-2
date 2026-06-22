import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span> <button>x</button>`, 1);
export default function App($$anchor, $$props) {
	let arr = $.prop($$props, "arr", 24, () => [{ prop: "foo" }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, arr, $.index, ($$anchor, o, $$index) => {
		var fragment_1 = root_1();
		var span = $.first_child(fragment_1);
		var text = $.child(span, true);
		$.reset(span);
		var button = $.sibling(span, 2);
		$.template_effect(() => $.set_text(text, (arr()[$$index], $.untrack(() => arr()[$$index].prop))));
		$.event("click", button, () => (arr()[$$index] = {
			...arr()[$$index],
			prop: "bar"
		}, $.invalidate_inner_signals(() => arr())));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
