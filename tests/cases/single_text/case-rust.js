import * as $ from "svelte/internal/client";
var root = $.template(`some_text`, 1);
export default function App($$anchor) {
	$.next();
	var fragment = root();
	$.append($$anchor, fragment);
}
