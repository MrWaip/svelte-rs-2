import * as $ from "svelte/internal/client";
var root = $.from_html(`<img src="a" alt="" loading="lazy"/> <div> <img src="b" alt="" loading="lazy"/></div>`, 1);
export default function App($$anchor) {
	let x = 1;
	var fragment = root();
	var img = $.first_child(fragment);
	var div = $.sibling(img, 2);
	var text = $.child(div, true);
	text.nodeValue = "1";
	var img_1 = $.sibling(text);
	$.reset(div);
	$.append($$anchor, fragment);
}
