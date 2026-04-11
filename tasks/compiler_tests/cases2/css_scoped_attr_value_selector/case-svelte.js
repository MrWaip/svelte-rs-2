import * as $ from "svelte/internal/client";
var root = $.from_html(`<div data-state="on" class="svelte-85638w">exact</div> <div data-state="On" class="svelte-85638w">insensitive</div> <div data-state="off">off</div> <button type="button" aria-label="run" class="svelte-85638w">button</button>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(6);
	$.append($$anchor, fragment);
}
