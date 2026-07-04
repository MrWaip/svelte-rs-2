import * as $ from "svelte/internal/client";
const greeting = ($$anchor, msg = $.noop) => {
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${msg() ?? ""}`));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let name = "world";
	var p_1 = root_1();
	p_1.textContent = "world";
	$.append($$anchor, p_1);
}
