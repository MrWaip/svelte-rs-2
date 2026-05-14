import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let count = $.state(0);
	function inc() {
		$.set(count, $.get(count) + 1);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, inc);
	$.append($$anchor, button);
}
$.delegate(["click"]);
