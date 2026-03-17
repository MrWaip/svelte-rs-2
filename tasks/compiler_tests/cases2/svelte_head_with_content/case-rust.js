import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="A great app"/>`);
var root = $.from_html(`<h1>Hello</h1>`);
export default function App($$anchor) {
	var h1 = root();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.append($$anchor, meta);
	});
	$.append($$anchor, h1);
}
