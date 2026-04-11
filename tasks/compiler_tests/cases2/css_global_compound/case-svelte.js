import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-1jl4a2j"><div class="b"><div class="c">compound</div></div></div> <section class="svelte-1jl4a2j"><strong>bare global</strong></section> <h1 class="title">title</h1>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
}
