import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp, 3)), a = $.mutable_source($.get($$array)[0]), c = $.mutable_source($.get($$array)[2]);
	function bump() {
		$.set(a, $.get(a));
		$.set(c, $.get(c));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(c) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
