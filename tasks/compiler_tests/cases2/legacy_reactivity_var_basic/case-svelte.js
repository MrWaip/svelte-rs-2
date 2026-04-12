import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	var count = $.mutable_source(0);
	function increment() {
		$.set(count, $.safe_get(count) + 1);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `clicks: ${$.safe_get(count) ?? ""}`));
	$.delegated("click", button, increment);
	$.append($$anchor, button);
}
$.delegate(["click"]);
