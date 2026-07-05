import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tmp = {}, a = $.tag($.mutable_source($.fallback(tmp.a, 10)), "a"), b = $.tag($.mutable_source($.fallback(tmp.b, 20)), "b");
	function bump() {
		$.set(a, $.get(a));
		$.set(b, $.get(b));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
	return $.pop($$exports);
}
