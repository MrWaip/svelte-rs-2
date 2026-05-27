import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.state(0);
	let z = $.state(0);
	let arr = [
		1,
		2,
		3
	];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.set(x, $$array[0], true);
			$.set(z, $.fallback($$array.slice(1).z, 26), true);
		})(arr);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(z) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
}
$.delegate(["click"]);
