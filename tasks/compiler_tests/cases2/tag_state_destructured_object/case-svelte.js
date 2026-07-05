import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = { pair: [1, 2] }, $$array = $.derived(() => $.to_array(tmp.pair, 2)), a = $.state($.proxy($.get($$array)[0])), b = $.state($.proxy($.get($$array)[1]));
	$.set(a, 10);
	$.set(b, 20);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${$.get(b) ?? ""}`));
	$.append($$anchor, p);
}
