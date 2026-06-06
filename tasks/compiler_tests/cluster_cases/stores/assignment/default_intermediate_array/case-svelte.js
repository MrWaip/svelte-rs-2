import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = $.mutable_source();
	const b = $.mutable_source();
	const c = $.mutable_source();
	const s = writable([[1, 2], 3]);
	$.legacy_pre_effect(() => ($.get(a), $.get(b), $.get(c), $s()), () => {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			var $$array_1 = $.to_array($.fallback($$array[0], () => [8, 9], true), 2);
			$.set(a, $$array_1[0]);
			$.set(b, $$array_1[1]);
			$.set(c, $$array[1]);
		})($s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}`));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
