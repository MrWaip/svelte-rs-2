import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { foo } from "./utils";
var root = $.add_locations($.from_html(`<input type="text"/>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => ($.deep_read_state(foo), $.untrack(() => foo.bar)), $.index, ($$anchor, bar, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, function get() {
			return $.get(bar).value;
		}, function set($$value) {
			$.get(bar).value = $$value, $.invalidate_inner_signals(() => foo);
		});
		$.append($$anchor, input);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
