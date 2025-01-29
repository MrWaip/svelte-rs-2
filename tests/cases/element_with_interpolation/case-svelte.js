import * as $ from "svelte/internal/client";
var root = $.template(`<div> </div> <div></div>`, 1);
export default function App($$anchor) {
	let title = 10;
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.child(div, true);
	$.reset(div);
	$.next(2);
	$.template_effect(() => $.set_text(text, title));
	$.append($$anchor, fragment);
}
