import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rows = $.prop($$props, "rows", 8);
	let key = $.prop($$props, "key", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, $.index, ($$anchor, $$item) => {
		let value = () => $.get($$item)[key()];
		value();
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, function get() {
			return value();
		}, function set($$value) {
			$.get($$item)[key] = $$value, $.invalidate_inner_signals(() => rows());
		});
		$.append($$anchor, input);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
