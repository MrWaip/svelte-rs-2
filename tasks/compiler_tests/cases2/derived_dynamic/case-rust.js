import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let count = $.state(0);
	let doubled = $.derived(() => $.get(count) * 2);
	function increment() {
		$.update(count);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.delegated("click", button, increment);
	$.append($$anchor, button);
}
$.delegate(["click"]);
