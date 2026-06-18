import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.prop($$props, "foo", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(foo()), $.untrack(() => foo().bar)), $.index, ($$anchor, bar, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => ($.deep_read_state(foo()), $.untrack(() => foo().bar))[$$index], ($$value) => (($.deep_read_state(foo()), $.untrack(() => foo().bar))[$$index] = $$value, $.invalidate_inner_signals(() => foo())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
	$.pop();
}
