import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const doubled = $.mutable_source(void 0, true);
	let x = $.prop($$props, "x", 9, 1);
	$.legacy_pre_effect(() => $.deep_read_state(x()), () => {
		$.set(doubled, x() * 2);
	});
	$.legacy_pre_effect_reset();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.append($$anchor, p);
	$.pop();
}
