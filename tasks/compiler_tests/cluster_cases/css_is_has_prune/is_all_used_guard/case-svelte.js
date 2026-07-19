import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<x class="svelte-1fgbrwj"><y class="svelte-1fgbrwj">y</y><z class="svelte-1fgbrwj">z</z></x>`);
export default function App($$anchor) {
	var x = root();
	$.append($$anchor, x);
}
