import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let obj = $.mutable_source({ x: 0 });
	function bump() {
		$.mutate(obj, $.get(obj).x += 1);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(obj).x + 1));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
}
$.delegate(["click"]);
