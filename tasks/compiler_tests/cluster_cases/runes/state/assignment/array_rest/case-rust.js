import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.state(0);
	let b = $.state(0);
	let arr = [
		1,
		2,
		3
	];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.set(a, $$array[0], true);
			$.set(b, $$array.slice(1), true);
		})(arr);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
}
$.delegate(["click"]);
