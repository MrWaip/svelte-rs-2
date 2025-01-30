import * as $ from "svelte/internal/client";
var root = $.template(`<div>text</div><div>text</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
}
