import * as $ from "svelte/internal/client";
var root = $.template(`<div>text only</div> <div></div> <div></div> <div><div>more nested</div> <div>more nested</div> <div>more nested</div></div> <div></div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var div = $.sibling($.first_child(fragment), 2);
	div.textContent = interpolation;
	var div_1 = $.sibling(div, 2);
	div_1.textContent = `concatenated + ${interpolation ?? ""} + concatenated`;
	$.next(4);
	$.next(4);
	$.append($$anchor, fragment);
}
