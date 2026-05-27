import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const k = "z";
	let { [k]: v } = { z: 1 };
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
