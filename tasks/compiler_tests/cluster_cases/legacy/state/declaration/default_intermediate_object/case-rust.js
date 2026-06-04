import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {}, a = $.mutable_source($.fallback(tmp.p, () => ({}), true).a);
	function bump() {
		$.set(a, $.get(a));
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
