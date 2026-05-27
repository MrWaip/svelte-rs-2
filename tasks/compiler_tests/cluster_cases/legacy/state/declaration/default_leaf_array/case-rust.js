import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let [a = 10, b = 20] = [1];
	function bump() {
		$.set(a, $.get(a));
		$.set(b, $.get(b));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
