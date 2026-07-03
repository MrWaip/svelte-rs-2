import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="h svelte-16f7pvy"><p class="svelte-16f7pvy">x</p></div>`);
export default function App($$anchor) {
	$.head("q2w0q4", ($$anchor) => {
		var div = root();
		$.append($$anchor, div);
	});
}
