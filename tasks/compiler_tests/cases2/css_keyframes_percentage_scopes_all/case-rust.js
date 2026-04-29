import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>plain</p> <button>btn</button>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
