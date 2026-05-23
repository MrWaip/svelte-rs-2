import * as $ from "svelte/internal/client";
var root = $.from_html(`<div data-a="&amp;amp=q &lt; ">a</div> <div data-b="© &amp;reg=x > foo">b</div> <div data-c="&amp;ok &amp;=q">c</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
}
