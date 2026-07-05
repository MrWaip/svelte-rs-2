import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const filtered = $.mutable_source();
	let items = $.tag($.mutable_source([{ done: false }]), "items");
	$.legacy_pre_effect(() => $.get(items), () => {
		$.set(filtered, $.get(items));
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
			$.get(item).done = $$value, $.invalidate_inner_signals(() => ($.get(filtered), $.get(items)));
		});
		$.append($$anchor, input);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
