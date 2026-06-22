import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { foo } from "lib";
var root = $.from_html(`<input/> <input/>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let obj = $.mutable_source({});
	let c = $.mutable_source("");
	$.legacy_pre_effect(() => ($.get(obj), $.get(c), foo), () => {
		$.mutate(obj, $.get(obj).purpose = ($.get(c) ? $.get(c) : "") + foo($.get(obj).type));
	});
	$.legacy_pre_effect_reset();
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	$.bind_value(input, () => $.get(c), ($$value) => $.set(c, $$value));
	$.bind_value(input_1, () => $.get(obj).x, ($$value) => $.mutate(obj, $.get(obj).x = $$value));
	$.append($$anchor, fragment);
	$.pop();
}
