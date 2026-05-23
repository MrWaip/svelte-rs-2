import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const id = $.mutable_source();
	const binding_group = [];
	let value = $.prop($$props, "value", 8);
	let group = $.prop($$props, "group", 12);
	$.legacy_pre_effect(() => $.deep_read_state(value()), () => {
		$.set(id, `${value()}-radio`);
	});
	$.legacy_pre_effect_reset();
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
	}, group);
	$.append($$anchor, input);
	$.pop();
}
