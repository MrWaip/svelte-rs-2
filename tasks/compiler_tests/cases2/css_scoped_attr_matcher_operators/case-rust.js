import * as $ from "svelte/internal/client";
var root = $.from_html(`<div data-tags="card active" class="svelte-v4glr4">class</div> <div data-lang="en-US" class="svelte-v4glr4">lang</div> <div data-url="https://example.com" class="svelte-v4glr4">href</div> <span data-tags="inactive">no class</span> <div data-lang="bengali">no lang</div> <div data-url="http://sample.org">no href</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(10);
	$.append($$anchor, fragment);
}
