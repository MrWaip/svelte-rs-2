import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<input/> <p> </p>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $value = () => ($.validate_store($.get(value), "value"), $.store_get($.get(value), "$value", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = $.mutable_source();
	let value = $.tag($.mutable_source(writable("")), "value");
	$.legacy_pre_effect(() => $value(), () => {
		$.set(x, $value());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var p = $.sibling(input, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.bind_value(input, function get() {
		return $.get(value);
	}, function set($$value) {
		$.store_unsub($.set(value, $$value), "$value", $$stores);
	});
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
