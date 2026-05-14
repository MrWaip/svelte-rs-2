import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let prop = $.prop($$props, "prop", 8);
	let local = $.mutable_source(0);
	let out = $.mutable_source(0);
	$.legacy_pre_effect(() => ($.get(local), $.deep_read_state(prop())), () => {
		if (true) {
			$.set(local, 1);
			$.set(out, (prop() || 0) + $.get(local));
		}
	});
	$.legacy_pre_effect_reset();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(out)));
	$.append($$anchor, p);
	$.pop();
}
