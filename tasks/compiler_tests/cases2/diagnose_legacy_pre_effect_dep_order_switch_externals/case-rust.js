import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { A } from "./a";
import { B } from "./b";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const y = $.mutable_source();
	let x = $.prop($$props, "x", 8);
	$.legacy_pre_effect(() => ($.deep_read_state(x()), B, A), () => {
		$.set(y, (function() {
			switch (x()) {
				case A.ONE: return B;
				default: return "";
			}
		})());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(y)));
	$.append($$anchor, p);
	$.pop();
}
