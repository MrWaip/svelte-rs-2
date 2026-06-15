import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const done = $.mutable_source();
	const filtered = $.mutable_source();
	let items = $.mutable_source([{ done: false }]);
	let filter = $.prop($$props, "filter", 8, "all");
	$.legacy_pre_effect(() => $.get(items), () => {
		$.set(done, $.get(items).filter((i) => i.done));
	});
	$.legacy_pre_effect(() => ($.deep_read_state(filter()), $.get(items), $.get(done)), () => {
		$.set(filtered, filter() === "all" ? $.get(items) : $.get(done));
	});
	$.legacy_pre_effect_reset();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $.get(filtered), $.index, ($$anchor, item, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		$.bind_checked(input, () => $.get(item).done, ($$value) => ($.get(item).done = $$value, $.invalidate_inner_signals(() => ($.get(filtered), filter(), $.get(items), $.get(done)))));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
	$.pop();
}
