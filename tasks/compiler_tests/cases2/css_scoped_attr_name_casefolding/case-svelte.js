import * as $ from "svelte/internal/client";
var root = $.from_html(`<div data-role="banner" class="svelte-1k56wwr">banner</div> <button type="button" aria-label="run" class="svelte-1k56wwr">run</button> <svg viewBox="0 0 10 10" class="svelte-1k56wwr"></svg>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
}
