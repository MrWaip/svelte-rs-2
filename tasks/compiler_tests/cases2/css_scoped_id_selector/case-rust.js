import * as $ from "svelte/internal/client";
var root = $.from_html(`<div id="target" class="svelte-1hmcw10">a</div> <span>b</span>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
