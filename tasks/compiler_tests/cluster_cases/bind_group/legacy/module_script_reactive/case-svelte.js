import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export const meta = { title: "x" };
var root = $.from_html(`<input type="radio"/> <input type="radio"/> <span> </span>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const binding_group = [];
	let one = $.mutable_source(1);
	let doubled = $.mutable_source(0);
	$.legacy_pre_effect(() => $.get(one), () => {
		$.set(doubled, $.get(one) * 2);
	});
	$.legacy_pre_effect_reset();
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
	}, ($$value) => $.set(one, $$value));
	$.bind_group(binding_group, [], input_1, () => {
		2;
		return $.get(one);
	}, ($$value) => $.set(one, $$value));
	$.append($$anchor, fragment);
	$.pop();
}
