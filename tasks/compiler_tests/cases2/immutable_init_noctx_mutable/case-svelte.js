import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><h3> </h3></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let count = $.prop($$props, "count", 12, 0);
	let foo = $.prop($$props, "foo", 24, () => ({ bar: "baz" }));
	$.legacy_pre_effect(() => ($.deep_read_state(foo()), $.deep_read_state(count())), () => {
		if (foo()) count(count() + 1);
	});
	$.legacy_pre_effect_reset();
	var div = root();
	var h3 = $.child(div);
	var text = $.child(h3);
	$.reset(h3);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Called ${count() ?? ""} times.`));
	$.append($$anchor, div);
	$.pop();
}
