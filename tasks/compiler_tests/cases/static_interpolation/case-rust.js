import * as $ from "svelte/internal/client";
var root = $.template(` <div><br> </div> <div> </div> <div><br> </div> <div> </div> `, 1);
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
	var text_2 = $.child(div_1, true);
	text_2.textContent = title;
	$.reset(div_1);
	var div_2 = $.sibling(div_1, 2);
	var text_3 = $.sibling($.child(div_2));
	text_3.nodeValue = ` ${title ?? ""} + ${title ?? ""} = x2${title ?? ""}`;
	$.reset(div_2);
	var div_3 = $.sibling(div_2, 2);
	var text_4 = $.child(div_3);
	text_4.textContent = `${title ?? ""} + ${title ?? ""} = x2${title ?? ""}`;
	$.reset(div_3);
	var text_5 = $.sibling(div_3);
	text_5.nodeValue = ` ${title ?? ""} + ${title ?? ""} = x2${title ?? ""}`;
	$.append($$anchor, fragment);
}
