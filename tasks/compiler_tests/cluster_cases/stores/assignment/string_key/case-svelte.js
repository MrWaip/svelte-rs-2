import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const ab = $.mutable_source();
	const cd = $.mutable_source();
	const s = writable({
		"a-b": 1,
		"c d": 2
	});
	$.legacy_pre_effect(() => ($.get(ab), $.get(cd), $s()), () => {
		(($$value) => {
			$.set(ab, $$value["a-b"]);
			$.set(cd, $$value["c d"]);
		})($s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
