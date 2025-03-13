import * as $ from "svelte/internal/client";
var root = $.template(`<section><span><span></span></span> <div><div><div>text</div></div></div> <p><b><i></i></b></p></section>`);
export default function App($$anchor) {
	var section = root();
	var span = $.child(section);
	var span_1 = $.child(span);
	span_1.textContent = name;
	$.reset(span);
	var p = $.sibling(span, 4);
	var b = $.child(p);
	var i = $.child(b);
	$.set_attribute(i, "name", name);
	$.reset(b);
	$.reset(p);
	$.reset(section);
	$.append($$anchor, section);
}
