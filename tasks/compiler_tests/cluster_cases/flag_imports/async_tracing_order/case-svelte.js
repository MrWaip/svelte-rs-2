import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let count = $.state(0);
	function read() {
		return $.get(count);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, $0), [() => read()]);
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
}
$.delegate(["click"]);
