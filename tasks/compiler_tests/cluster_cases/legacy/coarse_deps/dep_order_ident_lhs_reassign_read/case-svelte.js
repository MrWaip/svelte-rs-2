import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let x = $.mutable_source(0);
	let c = $.mutable_source(0);
	$.legacy_pre_effect(() => ($.get(x), $.get(c)), () => {
		$.set(x, $.get(x) + $.get(c));
	});
	$.legacy_pre_effect_reset();
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $.get(c), ($$value) => $.set(c, $$value));
	$.append($$anchor, input);
	$.pop();
}
