import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<meta name="description" content="A"/>`);
var root_1 = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var div = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.append($$anchor, meta);
	});
	$.append($$anchor, div);
}
