import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		p: [1, 2],
		q: [3, 4]
	}, $$array = $.derived(() => $.to_array(tmp.p, 2)), $$array_1 = $.derived(() => $.to_array(tmp.q, 2)), a = $.tag($.mutable_source($.get($$array)[0]), "a"), b = $.tag($.mutable_source($.get($$array)[1]), "b"), c = $.tag($.mutable_source($.get($$array_1)[0]), "c"), d = $.tag($.mutable_source($.get($$array_1)[1]), "d");
	function bump() {
		$.set(a, $.get(a));
		$.set(b, $.get(b));
		$.set(c, $.get(c));
		$.set(d, $.get(d));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
