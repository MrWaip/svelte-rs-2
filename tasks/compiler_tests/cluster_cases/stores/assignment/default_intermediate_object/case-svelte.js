import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = $.mutable_source();
	const s = writable({});
	$.legacy_pre_effect(() => ($.get(a), $s()), () => {
		(($$value) => {
			$.set(a, $.fallback($$value.p, () => ({}), true).a);
		})($s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
