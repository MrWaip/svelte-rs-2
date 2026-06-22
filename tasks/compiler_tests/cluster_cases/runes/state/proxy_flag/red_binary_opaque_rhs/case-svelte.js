import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let value = $.state(0);
	function add() {
		$.set(value, $.get(value) + 1);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, add);
	$.append($$anchor, button);
}
$.delegate(["click"]);
