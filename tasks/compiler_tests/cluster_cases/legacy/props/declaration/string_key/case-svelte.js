import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let tmp = {
		"a-b": 1,
		"c d": 2
	}, ab = $.prop($$props, "ab", 24, () => tmp["a-b"]), cd = $.prop($$props, "cd", 24, () => tmp["c d"]);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${ab() ?? ""}${cd() ?? ""}`));
	$.append($$anchor, button);
}
