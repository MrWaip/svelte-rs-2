import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.state(0);
	let obj = { x: 0 };
	let arr = [1, 2];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr, 2);
			obj.x = $$array[0];
			$.set(a, $$array[1], true);
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
