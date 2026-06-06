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
	const s = writable({
		a: 1,
		b: 2,
		c: 3
	});
	$.legacy_pre_effect(() => ($.get(a), $.get(rest), $s()), () => {
		(($$value) => {
			$.set(a, $$value.a);
			$.set(rest, $.exclude_from_object($$value, ["a"]));
		})($s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${($.get(rest), $.untrack(() => $.get(rest).b)) ?? ""}`));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
