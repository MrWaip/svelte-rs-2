App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state(""), "value");
	function action(node) {}
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.action(input, ($$node) => action?.($$node));
	$.effect(() => $.bind_value(input, function get() {
		return $.get(value);
	}, function set($$value) {
		$.set(value, $$value);
	}));
	$.append($$anchor, input);
	return $.pop($$exports);
}
