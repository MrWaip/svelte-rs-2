import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="A"/>`);
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.append($$anchor, meta);
	});
	$.append($$anchor, div);
}
