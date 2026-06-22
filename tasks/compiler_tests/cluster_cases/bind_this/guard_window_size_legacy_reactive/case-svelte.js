import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const doubled = $.mutable_source();
	let width = $.mutable_source(0);
	$.legacy_pre_effect(() => $.get(width), () => {
		$.set(doubled, $.get(width) * 2);
	});
	$.legacy_pre_effect_reset();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.bind_window_size("innerWidth", ($$value) => $.set(width, $$value));
	$.append($$anchor, p);
	$.pop();
}
