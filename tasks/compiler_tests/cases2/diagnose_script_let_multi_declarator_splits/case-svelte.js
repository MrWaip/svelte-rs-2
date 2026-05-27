import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = 1;
	let b = 2;
	function bump() {
		a += 1;
		b += 1;
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}-${b ?? ""}`));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
}
$.delegate(["click"]);
