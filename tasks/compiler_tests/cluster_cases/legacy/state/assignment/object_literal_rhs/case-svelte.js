import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.mutable_source(0);
	let b = $.mutable_source(0);
	function update() {
		(($$value) => {
			$.set(a, $$value.a);
			$.set(b, $$value.b);
		})({
			a: 1,
			b: 2
		});
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.event("click", button, update);
	$.append($$anchor, button);
}
