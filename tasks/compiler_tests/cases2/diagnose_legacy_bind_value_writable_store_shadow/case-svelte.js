import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<input/> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $value = () => $.store_get($.get(value), "$value", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = $.mutable_source();
	let value = $.mutable_source(writable(""));
	$.legacy_pre_effect(() => $value(), () => {
		$.set(x, $value());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var p = $.sibling(input, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(x)));
	$.bind_value(input, () => $.get(value), ($$value) => $.store_unsub($.set(value, $$value), "$value", $$stores));
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
