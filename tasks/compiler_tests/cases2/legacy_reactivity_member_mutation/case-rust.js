import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let object = $.mutable_source({ x: 0 });
	function bump() {
		$.mutate(object, $.get(object).x += 1);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `value: ${($.get(object), $.untrack(() => $.get(object).x)) ?? ""}`));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
}
$.delegate(["click"]);
