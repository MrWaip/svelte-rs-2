import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {
		"a-b": 1,
		"c d": 2
	}, ab = $.proxy(tmp["a-b"]), cd = $.proxy(tmp["c d"]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${ab ?? ""}${cd ?? ""}`));
	$.append($$anchor, button);
}
