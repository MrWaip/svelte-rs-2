import * as $ from "svelte/internal/client";
var root = $.template(`<div></div>`, 1);
export default function App($$anchor) {
	const title = "world";
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	text.nodeValue = `${title ?? ""} `;
	div.textContent = title;
	$.append($$anchor, fragment);
}
