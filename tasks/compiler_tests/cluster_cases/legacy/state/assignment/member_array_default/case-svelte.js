import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const a = 100;
	const arr = $.mutable_source([{ a: 1 }, 2]);
	function go() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.mutate(arr, $.get(arr)[0].a = $$array[0]);
			$.mutate(arr, $.get(arr)[1] = $.fallback($$array[1], a));
		})([$.get(arr)[1]]);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, $0), [() => ($.get(arr), $.untrack(() => JSON.stringify($.get(arr))))]);
	$.event("click", button, go);
	$.append($$anchor, button);
}
