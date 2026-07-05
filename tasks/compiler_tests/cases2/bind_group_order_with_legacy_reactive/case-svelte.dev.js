import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const id = $.mutable_source();
	const binding_group = [];
	let value = $.prop($$props, "value", 8);
	let group = $.prop($$props, "group", 12);
	$.legacy_pre_effect(() => $.deep_read_state(value()), () => {
		$.set(id, `${value()}-radio`);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	var input_value;
	$.template_effect(() => {
		$.set_attribute(input, "id", $.get(id));
		if (input_value !== (input_value = value())) {
			input.value = (input.__value = value()) ?? "";
		}
	});
	$.bind_group(binding_group, [], input, () => {
		value();
		return group();
	}, function set($$value) {
		group($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
