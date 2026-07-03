import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const filtered = $.mutable_source();
	let items = $.mutable_source([{ done: false }]);
	$.legacy_pre_effect(() => $.get(items), () => {
		$.set(filtered, $.get(items));
	});
	$.legacy_pre_effect_reset();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(filtered), $.index, ($$anchor, item, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_checked(input, () => $.get(item).done, ($$value) => ($.get(item).done = $$value, $.invalidate_inner_signals(() => ($.get(filtered), $.get(items)))));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
	$.pop();
}
