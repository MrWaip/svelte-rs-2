import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="asdf svelte-1t3la31"></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
