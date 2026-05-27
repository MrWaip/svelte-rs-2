import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.state(0);
	let arr = [];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr, 1);
			$.set(a, $.fallback($$array[0], () => ({}), true).a, true);
		})(arr);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.delegated("click", button, update);
	$.append($$anchor, button);
}
$.delegate(["click"]);
