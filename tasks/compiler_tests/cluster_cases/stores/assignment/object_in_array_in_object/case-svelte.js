import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const inner = $.mutable_source();
	const s = writable({ outer: [{ inner: 1 }] });
	$.legacy_pre_effect(() => ($.get(inner), $s()), () => {
		(($$value) => {
			var $$array = $.to_array($$value.outer, 1);
			$.set(inner, $$array[0].inner);
		})($s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(inner)));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
