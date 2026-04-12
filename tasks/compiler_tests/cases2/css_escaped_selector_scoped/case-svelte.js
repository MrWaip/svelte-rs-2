import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="foo:bar svelte-os1qct">class</div> <div id="hero:id" class="svelte-os1qct">id</div> <div class="miss">outside</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
}
