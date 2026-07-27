import * as $ from "svelte/internal/client";
var root = $.from_html(`<p dir="rtl">text</p> <div> <p dir="auto">dynamic parent reset</p></div>`, 1);
export default function App($$anchor) {
	let x = 1;
	var fragment = root();
	var p = $.first_child(fragment);
	var div = $.sibling(p, 2);
	var text = $.child(div, true);
	text.nodeValue = "1";
	var p_1 = $.sibling(text);
	$.reset(div);
	$.template_effect(() => {
		p.dir = p.dir;
		p_1.dir = p_1.dir;
	});
	$.append($$anchor, fragment);
}
