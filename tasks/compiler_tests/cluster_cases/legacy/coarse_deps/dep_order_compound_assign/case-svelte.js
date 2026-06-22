import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { foo } from "lib";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let c = $.mutable_source(0);
	$.legacy_pre_effect(() => ($.get(c), foo), () => {
		$.set(c, $.get(c) + foo());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $.get(c), ($$value) => $.set(c, $$value));
	$.append($$anchor, input);
	$.pop();
}
