import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<x class="svelte-u1v5md"><y class="svelte-u1v5md">y</y><z class="svelte-u1v5md">z</z></x>`);
export default function App($$anchor) {
	var x = root();
	$.append($$anchor, x);
}
