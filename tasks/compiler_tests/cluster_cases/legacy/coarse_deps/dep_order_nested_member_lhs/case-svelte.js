import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import { foo } from "lib";
var root = $.from_html(`<input/>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const obj = writable({});
	let c = $.mutable_source("");
	$.legacy_pre_effect(() => ($obj(), $.get(c), foo), () => {
		$.store_mutate(obj, $.untrack($obj).a.b = ($.get(c) ? $.get(c) : "") + foo($obj().x), $.untrack($obj));
	});
	$.legacy_pre_effect_reset();
	$.init();
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $.get(c), ($$value) => $.set(c, $$value));
	$.append($$anchor, input);
	$.pop();
	$$cleanup();
}
