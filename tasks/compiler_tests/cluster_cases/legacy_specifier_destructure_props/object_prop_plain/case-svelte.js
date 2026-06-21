import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = {
		a: 1,
		b: 2
	}, a = $.prop($$props, "a", 28, () => tmp.a), b = tmp.b;
	function inc() {
		$.update_prop(a);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b ?? ""}`));
	$.event("click", button, inc);
	$.append($$anchor, button);
}
