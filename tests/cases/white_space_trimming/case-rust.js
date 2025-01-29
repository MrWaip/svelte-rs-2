import * as $ from "svelte/internal/client";
var root = $.template(`start<br> <br>end`, 1);
export default function App($$anchor) {
	let name = 12;
	$.next();
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
}
