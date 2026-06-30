import * as $ from "svelte/internal/client";
var root = $.from_svg(`<svg></svg><svg></svg>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
}
