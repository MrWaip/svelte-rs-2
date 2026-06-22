import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.mutable_source(0);
	let bar = $.mutable_source(0);
	$.set(foo, 4);
	$.legacy_pre_effect(() => ($.get(bar), $.get(foo)), () => {
		$.set(bar, 0);
		outer: for (let i = 0; i < $.get(foo); i++) {
			for (let j = 0; j < $.get(foo); j++) {
				if (j > i) break outer;
				$.set(bar, $.get(bar) + j);
			}
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
