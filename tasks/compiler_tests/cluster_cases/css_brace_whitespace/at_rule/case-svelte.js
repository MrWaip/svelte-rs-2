import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="foo svelte-i6qdor">d</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
