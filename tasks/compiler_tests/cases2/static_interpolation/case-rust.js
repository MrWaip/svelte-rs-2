import * as $ from "svelte/internal/client";
var root = $.from_html(` <div><br/> </div> <div></div>`, 1);
export default function App($$anchor) {
	const title = "world";
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	text.nodeValue = "world ";
	var div = $.sibling(text);
	var text_1 = $.sibling($.child(div));
	text_1.nodeValue = " world";
	$.reset(div);
	var div_1 = $.sibling(div, 2);
	div_1.textContent = "world";
	$.append($$anchor, fragment);
}
