import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {
		a: 1,
		b: 2
	}, x = $.mutable_source(tmp.a), y = $.mutable_source(tmp.b);
	function bump() {
		$.set(x, $.get(x));
		$.set(y, $.get(y));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(y) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
