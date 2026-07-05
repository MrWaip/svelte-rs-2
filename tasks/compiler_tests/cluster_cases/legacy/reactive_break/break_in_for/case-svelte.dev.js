import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1> </h1>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.tag($.mutable_source(0), "foo");
	let bar = $.tag($.mutable_source(0), "bar");
	$.set(foo, 5);
	$.legacy_pre_effect(() => ($.get(bar), $.get(foo)), () => {
		$.set(bar, 0);
		for (let i = 0; i < $.get(foo); i++) {
			$.set(bar, $.get(bar) + i);
			if (i > 2) return;
		}
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var h1 = root();
	var text = $.child(h1);
	$.reset(h1);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""} ${$.get(bar) ?? ""}`));
	$.append($$anchor, h1);
	return $.pop($$exports);
}
