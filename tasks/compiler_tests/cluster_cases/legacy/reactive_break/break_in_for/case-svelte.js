import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.mutable_source(0);
	let bar = $.mutable_source(0);
	$.set(foo, 5);
	$.legacy_pre_effect(() => ($.get(bar), $.get(foo)), () => {
		$.set(bar, 0);
		for (let i = 0; i < $.get(foo); i++) {
			$.set(bar, $.get(bar) + i);
			if (i > 2) return;
		}
	});
	$.legacy_pre_effect_reset();
	var h1 = root();
	var text = $.child(h1);
	$.reset(h1);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""} ${$.get(bar) ?? ""}`));
	$.append($$anchor, h1);
	$.pop();
}
