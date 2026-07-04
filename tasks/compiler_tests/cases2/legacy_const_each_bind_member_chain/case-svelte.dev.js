import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const obj = $.tag($.mutable_source({
		keys: ["a"],
		fields: { a: { value: 0 } }
	}), "obj");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => $.get(obj).keys, (key) => key, ($$anchor, key) => {
		const field = $.tag($.derived_safe_equal(() => $.get(obj).fields[$.get(key)]), "field");
		$.get(field);
		var input = root();
		$.remove_input_defaults(input);
		$.bind_value(input, function get() {
			return $.get(field).value;
		}, function set($$value) {
			$.get(field).value = $$value;
		});
		$.append($$anchor, input);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
