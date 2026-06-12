import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { foo } from "lib";
var root = $.from_html(`<input/> <input/>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let total = $.mutable_source(0);
	let c = $.mutable_source(0);
	let d = $.mutable_source(0);
	$.legacy_pre_effect(() => ($.get(c), foo, $.get(d)), () => {
		$.set(total, $.get(c) + foo($.get(d)));
	});
	$.legacy_pre_effect_reset();
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	$.bind_value(input, () => $.get(c), ($$value) => $.set(c, $$value));
	$.bind_value(input_1, () => $.get(d), ($$value) => $.set(d, $$value));
	$.append($$anchor, fragment);
	$.pop();
}
