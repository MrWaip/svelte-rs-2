import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let value = $.state("a");
	function toggle() {
		$.set(value, $.get(value) === "a" ? "b" : "c", true);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, toggle);
	$.append($$anchor, button);
}
$.delegate(["click"]);
