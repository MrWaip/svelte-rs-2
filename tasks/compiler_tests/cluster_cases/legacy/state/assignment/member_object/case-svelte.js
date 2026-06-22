import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let obj = $.mutable_source({ x: 0 });
	let src = { v: 1 };
	function go() {
		$.mutate(obj, $.get(obj).x = src.v);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($.get(obj), $.untrack(() => $.get(obj).x))));
	$.event("click", button, go);
	$.append($$anchor, button);
}
