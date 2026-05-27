import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = [[1, 2], [3, 4]], $$array = $.derived(() => $.to_array(tmp, 2)), $$array_1 = $.derived(() => $.to_array($.get($$array)[0], 2)), a = $.mutable_source($.get($$array_1)[0]), b = $.mutable_source($.get($$array_1)[1]), $$array_2 = $.derived(() => $.to_array($.get($$array)[1], 2)), c = $.mutable_source($.get($$array_2)[0]), d = $.mutable_source($.get($$array_2)[1]);
	function bump() {
		$.set(a, $.get(a));
		$.set(b, $.get(b));
		$.set(c, $.get(c));
		$.set(d, $.get(d));
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}${$.get(c) ?? ""}${$.get(d) ?? ""}`));
	$.event("click", button, bump);
	$.append($$anchor, button);
}
