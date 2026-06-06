import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = { outer: [{ inner: 1 }] }, $$array = $.derived(() => $.to_array(tmp.outer, 1)), inner = $.mutable_source($.get($$array)[0].inner);
	function bump() {
		$.set(inner, $.get(inner));
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(inner)));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
