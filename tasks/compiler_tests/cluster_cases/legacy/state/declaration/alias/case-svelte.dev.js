import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {
		a: 1,
		b: 2
	}, x = $.tag($.mutable_source(tmp.a), "x"), y = $.tag($.mutable_source(tmp.b), "y");
	function bump() {
		$.set(x, $.get(x));
		$.set(y, $.get(y));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(y) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
