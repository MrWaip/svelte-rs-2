import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.mutable_source(tmp.a), rest = $.exclude_from_object(tmp, ["a"]);
	function bump() {
		$.set(a, $.get(a));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.untrack(() => rest.b) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
