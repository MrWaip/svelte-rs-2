import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let value = 42;
	const double = (n) => n * 2;
	const get_value = () => value;
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => double(get_value())]);
	$.append($$anchor, p);
}
