import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="a svelte-1awuth">a</span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
