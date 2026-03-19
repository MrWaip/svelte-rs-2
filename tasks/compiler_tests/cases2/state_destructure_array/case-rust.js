import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let tmp = [1, 2], $$array = $.derived(() => $.to_array(tmp, 2)), x = $.state($.proxy($.get($$array)[0])), y = $.proxy($.get($$array)[1]);
	$.set(x, $.get(x) + 1);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""} ${y ?? ""}`));
	$.append($$anchor, p);
}
