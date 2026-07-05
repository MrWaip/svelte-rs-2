import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let model = $.prop($$props, "model", 12, "a");
	let value = $.prop($$props, "value", 8, "a");
	let name = $.prop($$props, "name", 8, "radio");
	let hasError = $.prop($$props, "hasError", 8, false);
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	let classes;
	var input_value;
	$.template_effect(() => {
		$.set_attribute(input, "name", name());
		classes = $.set_class(input, 1, "", null, classes, { error: hasError() });
		if (input_value !== (input_value = value())) {
			input.value = (input.__value = value()) ?? "";
		}
	});
	$.event("click", input, function($$arg) {
		$.bubble_event.call(this, $$props, $$arg);
	});
	$.bind_group(binding_group, [], input, () => {
		value();
		return model();
	}, function set($$value) {
		model($$value);
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
