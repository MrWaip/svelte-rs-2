import * as $ from "svelte/internal/client";
let shared = $.state(0);
let doubled = $.derived(() => $.get(shared) * 2);
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	function increment() {
		$.update(shared);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.delegated("click", button, increment);
	$.append($$anchor, button);
}
$.delegate(["click"]);
