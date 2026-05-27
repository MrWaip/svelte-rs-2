import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.mutable_source(0);
	let rest = $.mutable_source(0);
	let obj = {
		a: 1,
		x: 2
	};
	function update() {
		$.set(a, obj.a), $.set(rest, $.exclude_from_object(obj, ["a"]));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(rest) ?? ""}`));
	$.event("click", button, update);
	$.append($$anchor, button);
}
