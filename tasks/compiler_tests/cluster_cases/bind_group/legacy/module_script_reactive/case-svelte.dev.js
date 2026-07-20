import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export const meta = { title: "x" };
var root = $.add_locations($.from_html(`<input type="radio"/> <input type="radio"/> <span> </span>`, 1), App[$.FILENAME], [
	[11, 0],
	[12, 0],
	[13, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let one = $.tag($.mutable_source(1), "one");
	let doubled = $.tag($.mutable_source(0), "doubled");
	$.legacy_pre_effect(() => $.get(one), () => {
		$.set(doubled, $.get(one) * 2);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = 1;
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = 2;
	var span = $.sibling(input_1, 2);
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.bind_group(binding_group, [], input, () => {
		1;
		return $.get(one);
	}, function set($$value) {
		$.set(one, $$value);
	});
	$.bind_group(binding_group, [], input_1, () => {
		2;
		return $.get(one);
	}, function set($$value) {
		$.set(one, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
