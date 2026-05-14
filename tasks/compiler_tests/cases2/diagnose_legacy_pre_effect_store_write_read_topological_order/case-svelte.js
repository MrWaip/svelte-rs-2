import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $w = () => $.store_get(w, "$w", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const w = writable(0);
	let derived = $.mutable_source(0);
	$.legacy_pre_effect(() => {}, () => {
		(() => {
			$.store_set(w, 1);
		})();
	});
	$.legacy_pre_effect(() => $w(), () => {
		$.set(derived, $w() * 2);
	});
	$.legacy_pre_effect_reset();
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(derived)));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
