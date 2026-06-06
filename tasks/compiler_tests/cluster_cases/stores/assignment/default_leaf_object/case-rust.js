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
	const s = writable({});
	$.legacy_pre_effect(() => ($.get(a), $.get(b), $s()), () => {
		(($$value) => {
			$.set(a, $.fallback($$value.a, 10));
			$.set(b, $.fallback($$value.b, 20));
		})($s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
