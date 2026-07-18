import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="y svelte-178le0y"><span class="z svelte-178le0y">z</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
