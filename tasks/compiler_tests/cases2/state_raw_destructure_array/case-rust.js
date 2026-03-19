import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = [1, 2], $$array = $.derived(() => $.to_array(tmp, 2)), x = $.get($$array)[0], y = $.get($$array)[1];
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${x ?? ""} ${y ?? ""}`));
	$.append($$anchor, p);
}
