import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export const M = 1;
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 8, 0);
	let b = $.prop($$props, "b", 12, "");
	$.legacy_pre_effect(() => ($.deep_read_state(b()), $.deep_read_state(a())), () => {
		b(b() || (a() ? "x" : "y"));
	});
	$.legacy_pre_effect_reset();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, b()));
	$.append($$anchor, p);
	$.pop();
}
