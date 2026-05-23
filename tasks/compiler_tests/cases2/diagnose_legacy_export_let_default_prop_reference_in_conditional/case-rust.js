import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor, $$props) {
	let kind = $.prop($$props, "kind", 8, "a");
	let label = $.prop($$props, "label", 24, () => kind() === "a" ? "first" : "second");
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, label()));
	$.append($$anchor, div);
}
