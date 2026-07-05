import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.tag($.mutable_source(0), "x");
	let c = $.tag($.mutable_source(0), "c");
	$.legacy_pre_effect(() => ($.get(x), $.get(c)), () => {
		$.set(x, $.get(x) + $.get(c));
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, function get() {
		return $.get(c);
	}, function set($$value) {
		$.set(c, $$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
