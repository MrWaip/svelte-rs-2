import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	let $$array = $.derived(() => $.to_array(items)), first = $.derived(() => $.get($$array)[0]), second = $.derived(() => $.get($$array)[1]), rest = $.derived(() => $.get($$array).slice(2));
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(first) ?? ""},${$.get(second) ?? ""}`));
	$.append($$anchor, p);
}
