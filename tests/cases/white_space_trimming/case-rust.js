import * as $ from "svelte/internal/client";
var root = $.template(`<br><br>`, 1);
export default function App($$anchor) {
	let name = 12;
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
}
