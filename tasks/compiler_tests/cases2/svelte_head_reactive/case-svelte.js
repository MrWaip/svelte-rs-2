import * as $ from "svelte/internal/client";
var root = $.from_html(`<meta name="description"/>`);
export default function App($$anchor) {
	let count = 0;
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.set_attribute(meta, "content", count);
		$.append($$anchor, meta);
	});
}
