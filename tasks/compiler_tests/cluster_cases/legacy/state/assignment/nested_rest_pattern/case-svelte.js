import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.mutable_source(0);
	let z = $.mutable_source(0);
	let arr = [
		1,
		2,
		3
	];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.set(x, $$array[0]);
			$.set(z, $.fallback($$array.slice(1).z, 26));
		})(arr);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(z) ?? ""}`));
	$.event("click", button, update);
	$.append($$anchor, button);
}
