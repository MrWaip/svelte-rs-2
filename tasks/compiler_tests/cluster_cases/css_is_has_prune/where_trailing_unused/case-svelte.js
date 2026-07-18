import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<x class="svelte-zyrlym"><y class="svelte-zyrlym">y</y></x>`);
export default function App($$anchor) {
	var x = root();
	$.append($$anchor, x);
}
