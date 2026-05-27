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
	const s = writable([{ a: 1 }, { b: 2 }]);
	$.legacy_pre_effect(() => ($.get(a), $.get(b), $s()), () => {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(a, $$array[0].a);
			$.set(b, $$array[1].b);
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
