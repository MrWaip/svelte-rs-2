import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><h3> </h3></div>`), App[$.FILENAME], [[
	9,
	0,
	[[10, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.prop($$props, "count", 13, 0);
	let foo = $.prop($$props, "foo", 25, () => ({ bar: "baz" }));
	$.legacy_pre_effect(() => ($.deep_read_state(foo()), $.deep_read_state(count())), () => {
		if (foo()) count(count() + 1);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var h3 = $.child(div);
	var text = $.child(h3);
	$.reset(h3);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Called ${count() ?? ""} times.`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
