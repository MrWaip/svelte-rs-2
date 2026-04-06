import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="svelte-1rk0dqc">content</p> <p class="bar svelte-1rk0dqc">bar</p> <div class="svelte-1rk0dqc">box</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
}
