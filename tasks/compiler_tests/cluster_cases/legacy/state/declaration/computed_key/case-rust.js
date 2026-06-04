import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const k = "z";
	let tmp = { z: 1 }, v = $.mutable_source(tmp[k]);
	function bump() {
		$.set(v, $.get(v));
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(v)));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
