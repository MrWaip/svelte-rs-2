import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let value = $.state(0);
	function make() {
		return {};
	}
	function reset() {
		$.set(value, make(), true);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, reset);
	$.append($$anchor, button);
}
$.delegate(["click"]);
