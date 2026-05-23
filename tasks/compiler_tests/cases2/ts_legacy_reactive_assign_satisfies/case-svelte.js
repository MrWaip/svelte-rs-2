import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const doubled = $.mutable_source();
	let count = 0;
	$.legacy_pre_effect(() => {}, () => {
		$.set(doubled, count * 2);
	});
	$.legacy_pre_effect_reset();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.append($$anchor, p);
	$.pop();
}
