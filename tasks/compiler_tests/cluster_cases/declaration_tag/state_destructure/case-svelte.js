import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let tmp = $$props.o, a = $.proxy(tmp.a), b = $.proxy(tmp.b);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a ?? ""} ${b ?? ""}`));
	$.append($$anchor, p);
}
