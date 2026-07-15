import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.mutable_source();
	let b = $.mutable_source();
	const obj = {
		a: 1,
		b: 2
	};
	function run() {
		$.set(a, obj.a), $.set(b, obj.b);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
}
