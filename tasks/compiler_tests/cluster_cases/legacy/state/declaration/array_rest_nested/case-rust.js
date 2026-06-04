import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.derived(() => $.to_array(tmp)), $$array_1 = $.derived(() => $.to_array($.get($$array).slice(1), 2)), a = $.mutable_source($.get($$array)[0]), b = $.mutable_source($.get($$array_1)[0]), c = $.mutable_source($.get($$array_1)[1]);
	function bump() {
		$.set(a, $.get(a));
		$.set(b, $.get(b));
		$.set(c, $.get(c));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
