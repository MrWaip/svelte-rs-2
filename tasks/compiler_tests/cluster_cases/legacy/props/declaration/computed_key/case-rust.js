import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	const k = "z";
	let tmp = { z: 1 }, v = $.prop($$props, "v", 24, () => tmp[k]);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, v()));
	$.append($$anchor, button);
}
