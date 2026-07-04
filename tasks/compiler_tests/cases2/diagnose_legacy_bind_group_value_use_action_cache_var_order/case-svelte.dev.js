import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let model = $.prop($$props, "model", 12, "a");
	let value = $.prop($$props, "value", 8, "a");
	function action() {}
	var $$exports = { ...$.legacy_api() };
	var input = root();
	$.remove_input_defaults(input);
	$.effect(() => $.bind_group(binding_group, [], input, () => {
		value();
		return model();
	}, function set($$value) {
		model($$value);
	}));
	$.action(input, ($$node) => action?.($$node));
	var input_value;
	$.template_effect(() => {
		if (input_value !== (input_value = value())) {
			input.value = (input.__value = value()) ?? "";
		}
	});
	$.append($$anchor, input);
	return $.pop($$exports);
}
