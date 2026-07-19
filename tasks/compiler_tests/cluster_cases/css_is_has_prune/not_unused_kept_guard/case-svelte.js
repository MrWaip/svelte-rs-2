import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<x class="foo svelte-j6kmc3">f</x>`);
export default function App($$anchor) {
	var x = root();
	$.append($$anchor, x);
}
