import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let array = $.prop($$props, "array", 24, () => [{ value: "" }, {}]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, array, $.index, ($$anchor, $$item) => {
		let value = $.derived_safe_equal(() => $.fallback($.get($$item).value, "hello"));
		$.get(value);
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, function get() {
			return $.get(value);
		}, function set($$value) {
			$.get($$item).value = $$value, $.invalidate_inner_signals(() => array());
		});
		$.append($$anchor, input);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
