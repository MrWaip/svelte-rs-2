import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = [1, 2], $$array = $.derived(() => $.to_array(tmp, 2)), a = $.prop($$props, "a", 28, () => $.get($$array)[0]), b = $.prop($$props, "b", 28, () => $.get($$array)[1]);
	function inc() {
		$.update_prop(a);
		$.update_prop(b);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.event("click", button, inc);
	$.append($$anchor, button);
}
