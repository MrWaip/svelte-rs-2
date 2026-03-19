import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = {
		a: 1,
		b: 2,
		c: 3
	}, a = $.proxy(tmp.a), rest = $.proxy($.exclude_from_object(tmp, ["a"]));
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, a));
	$.append($$anchor, p);
}
