import * as $ from "svelte/internal/client";
var root = $.template(` <div><br> </div> <div></div>`, 1);
export default function App($$anchor) {
	const title = "world";
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	text.nodeValue = `${title ?? ""} `;
	var div = $.sibling(text);
	var text_1 = $.sibling($.child(div));
	text_1.nodeValue = ` ${title ?? ""}`;
	$.reset(div);
	var div_1 = $.sibling(div, 2);
	div_1.textContent = title;
	$.append($$anchor, fragment);
}
