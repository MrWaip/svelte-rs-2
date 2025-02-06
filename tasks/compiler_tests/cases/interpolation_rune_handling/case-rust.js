import * as $ from "svelte/internal/client";
var root = $.template(`<br> <br> <br> <div> </div>`, 1);
export default function App($$anchor) {
	let title = $.state(10);
	let name = "";
	$.set(title, 12);
	var fragment = root();
	var text = $.sibling($.first_child(fragment));
	text.textContent = ` ${name ?? ""} `;
	var text_1 = $.sibling(text, 2);
	text_1.textContent = ` ${$.get(title) ?? ""} `;
	var text_2 = $.sibling(text_1, 2);
	text_2.textContent = ` _${name ?? ""}_${$.get(title) ?? ""} `;
	var div = $.sibling(text_2);
	var text_3 = $.child(div);
	text_3.textContent = `${name ?? ""}+${$.get(title) ?? ""}`;
	$.reset(div);
	$.append($$anchor, fragment);
}
