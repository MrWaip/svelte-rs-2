import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.mutable_source(0);
	let key = "prop";
	let obj = { prop: 1 };
	function update() {
		$.set(a, obj[key]);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.event("click", button, update);
	$.append($$anchor, button);
}
