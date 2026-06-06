import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = { outer: [{ inner: 1 }] }, $$array = $.derived(() => $.to_array(tmp.outer, 1)), inner = $.prop($$props, "inner", 24, () => $.get($$array)[0].inner);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, inner()));
	$.append($$anchor, button);
}
