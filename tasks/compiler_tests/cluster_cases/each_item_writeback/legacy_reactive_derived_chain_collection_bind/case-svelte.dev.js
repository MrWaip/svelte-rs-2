import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const done = $.mutable_source();
	const filtered = $.mutable_source();
	let items = $.tag($.mutable_source([{ done: false }]), "items");
	let filter = $.prop($$props, "filter", 8, "all");
	$.legacy_pre_effect(() => $.get(items), () => {
		$.set(done, $.get(items).filter((i) => i.done));
	});
	$.legacy_pre_effect(() => ($.deep_read_state(filter()), $.get(items), $.get(done)), () => {
		$.set(filtered, $.strict_equals(filter(), "all") ? $.get(items) : $.get(done));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(filtered), $.index, ($$anchor, item, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_checked(input, function get() {
			return $.get(item).done;
		}, function set($$value) {
			$.get(item).done = $$value, $.invalidate_inner_signals(() => ($.get(filtered), filter(), $.get(items), $.get(done)));
		});
		$.append($$anchor, input);
	}), "each", App, 9, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
