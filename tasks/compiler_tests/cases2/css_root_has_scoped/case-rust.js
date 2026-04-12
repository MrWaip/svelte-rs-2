import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="hit svelte-17fn2ym">inside</div> <div>outside</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
