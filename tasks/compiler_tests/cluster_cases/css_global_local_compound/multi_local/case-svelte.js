import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="x y svelte-1p8xzv0">d</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
