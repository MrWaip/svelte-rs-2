import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="card svelte-mv1sf"><span class="inside svelte-mv1sf">inside</span></div> <span class="inside">outside</span>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
