import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = $.mutable_source();
	const rest = $.mutable_source();
	const s = writable([
		1,
		2,
		3
	]);
	$.legacy_pre_effect(() => ($.get(a), $.get(rest), $s()), () => {
		(($$value) => {
			var $$array = $.to_array($$value);
			$.set(a, $$array[0]);
			$.set(rest, $$array.slice(1));
		})($s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${($.get(rest), $.untrack(() => $.get(rest).length)) ?? ""}`));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
