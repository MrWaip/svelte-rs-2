import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.mutable_source(0);
	let bar = $.mutable_source();
	$.set(foo, 1);
	$.legacy_pre_effect(() => $.get(foo), () => {
		$.set(bar, $.get(foo) + 1);
		if ($.get(foo)) {
			return;
		}
		$.set(bar, $.get(foo) + 2);
	});
	$.legacy_pre_effect_reset();
	var h1 = root();
	var text = $.child(h1);
	$.reset(h1);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""} ${$.get(bar) ?? ""}`));
	$.append($$anchor, h1);
	$.pop();
}
