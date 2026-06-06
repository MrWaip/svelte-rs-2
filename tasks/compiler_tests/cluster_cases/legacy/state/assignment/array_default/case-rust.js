import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.mutable_source(0);
	let arr = [];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr, 1);
			$.set(a, $.fallback($$array[0], 5));
		})(arr);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.event("click", button, update);
	$.append($$anchor, button);
}
