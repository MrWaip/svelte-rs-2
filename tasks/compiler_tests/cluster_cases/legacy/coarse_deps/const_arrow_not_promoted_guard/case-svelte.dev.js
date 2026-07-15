import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
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
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, function get() {
			return $.get(item).a;
		}, function set($$value) {
			$.get(item).a = $$value, $.invalidate_inner_signals(() => items());
		});
		$.append($$anchor, input);
	}), "each", App, 9, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
