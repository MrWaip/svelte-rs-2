import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { foo } from "./utils";
var root_1 = $.from_html(`<input type="text"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($.deep_read_state(foo), $.untrack(() => foo.bar)), $.index, ($$anchor, bar, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(bar).value, ($$value) => ($.get(bar).value = $$value, $.invalidate_inner_signals(() => foo)));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
	$.pop();
}
