import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/> <textarea/> <area/> <br/> <a></a>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(8);
	$.append($$anchor, fragment);
}
