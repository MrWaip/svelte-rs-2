import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let model = $.prop($$props, "model", 12, "a");
	let value = $.prop($$props, "value", 8, "a");
	function action() {}
	var input = root();
	$.remove_input_defaults(input);
	var input_value;
	$.effect(() => $.bind_group(binding_group, [], input, () => {
		value();
		return model();
	}, model));
	$.action(input, ($$node) => action?.($$node));
	$.template_effect(() => {
		if (input_value !== (input_value = value())) {
			input.value = (input.__value = value()) ?? "";
		}
	});
	$.append($$anchor, input);
}
