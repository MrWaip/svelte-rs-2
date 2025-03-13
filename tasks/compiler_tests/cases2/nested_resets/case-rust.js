import * as $ from "svelte/internal/client";
var root = $.template(`<div><div><div></div></div> <div><div><div>text</div></div></div> <div><div><div></div></div></div></div>`);
export default function App($$anchor) {
	var div = root();
	var div_1 = $.child(div);
	var div_2 = $.child(div_1);
	div_2.textContent = name;
	$.reset(div_1);
	$.next(4);
	$.reset(div);
	$.append($$anchor, div);
}
