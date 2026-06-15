import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.state(1);
	let y = $.state(2);
	function inc() {
		$.update(x);
		$.update(y);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(y) ?? ""}`));
	$.delegated("click", button, inc);
	$.append($$anchor, button);
}
$.delegate(["click"]);
