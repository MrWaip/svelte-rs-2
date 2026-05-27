import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {
		"a-b": 1,
		"c d": 2
	}, ab = $.mutable_source(tmp.a-b), cd = $.mutable_source(tmp.c d);
	function bump() {
		$.set(ab, $.get(ab));
		$.set(cd, $.get(cd));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(ab) ?? ""}${$.get(cd) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
