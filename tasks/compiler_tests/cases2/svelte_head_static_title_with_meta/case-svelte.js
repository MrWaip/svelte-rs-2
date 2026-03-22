import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="A page"/>`);
export default function App($$anchor) {
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.effect(() => {
			$.document.title = "My Page";
		});
		$.append($$anchor, meta);
	});
}
