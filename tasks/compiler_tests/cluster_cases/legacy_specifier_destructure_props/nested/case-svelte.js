import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = { x: [1] }, $$array = $.derived(() => $.to_array(tmp.x, 1)), bar = $.prop($$props, "bar", 28, () => $.get($$array)[0]);
	function inc() {
		$.update_prop(bar);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, bar()));
	$.event("click", button, inc);
	$.append($$anchor, button);
}
