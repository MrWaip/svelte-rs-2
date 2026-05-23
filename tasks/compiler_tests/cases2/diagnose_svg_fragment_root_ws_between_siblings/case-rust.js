import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg><path d="M1"></path></svg><g><path d="M2"></path></g>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
}
