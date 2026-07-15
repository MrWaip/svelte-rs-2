import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let items = $.prop($$props, "items", 12);
	const makeEmpty = () => ({ a: "" });
	$.legacy_pre_effect(() => $.deep_read_state(items()), () => {
		if (items()) {
			items(items().map((x) => ({
				...x,
				info: x.info ?? makeEmpty()
			})));
		}
	});
	$.legacy_pre_effect_reset();
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, () => $.get(item).a, ($$value) => ($.get(item).a = $$value, $.invalidate_inner_signals(() => items())));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
	$.pop();
}
