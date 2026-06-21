import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = {
		a: 1,
		s: 2
	}, a = $.prop($$props, "a", 28, () => tmp.a), s = tmp.s;
	function inc() {
		$.update_prop(a);
		$.update(s);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${$.get(s) ?? ""}`));
	$.event("click", button, inc);
	$.append($$anchor, button);
}
