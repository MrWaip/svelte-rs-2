import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let d = $.prop($$props, "d", 12, 1);
	function inc() {
		$.update_prop(d);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, d()));
	$.event("click", button, inc);
	$.append($$anchor, button);
}
