import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [1], $$array = $.derived(() => $.to_array(tmp, 2)), a = $.mutable_source($.fallback($.get($$array)[0], 10)), b = $.mutable_source($.fallback($.get($$array)[1], 20));
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
