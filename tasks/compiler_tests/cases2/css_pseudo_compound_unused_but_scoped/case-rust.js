import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="foo svelte-mcqa8k">foo</p> <span class="baz svelte-mcqa8k">baz</span>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}
