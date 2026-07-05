import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { foo } from "lib";
var root = $.add_locations($.from_html(`<input/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let c = $.tag($.mutable_source(0), "c");
	$.legacy_pre_effect(() => ($.get(c), foo), () => {
		$.set(c, $.get(c) + foo());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
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
