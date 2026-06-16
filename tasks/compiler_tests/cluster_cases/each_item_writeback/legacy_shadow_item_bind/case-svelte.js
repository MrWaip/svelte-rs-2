import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(` <input/>`, 1);
export default function App($$anchor) {
	let a = $.mutable_source(["Hello"]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(a), $.index, ($$anchor, a, $$index, $$array) => {
		$.next();
		var fragment_1 = root_1();
		var text = $.first_child(fragment_1);
		var input = $.sibling(text);
		$.remove_input_defaults(input);
		$.template_effect(() => $.set_text(text, `${$$array()[$$index] ?? ""} `));
		$.bind_value(input, () => $$array()[$$index], ($$value) => ($$array()[$$index] = $$value, $.invalidate_inner_signals(() => $$array())));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
