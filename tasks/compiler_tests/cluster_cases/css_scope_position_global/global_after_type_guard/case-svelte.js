import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-1fwir9w">d</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
