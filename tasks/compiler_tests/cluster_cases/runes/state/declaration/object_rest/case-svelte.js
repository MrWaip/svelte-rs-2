import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.proxy(tmp.a), rest = $.proxy($.exclude_from_object(tmp, ["a"]));
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${rest.b ?? ""}`));
	$.append($$anchor, button);
}
