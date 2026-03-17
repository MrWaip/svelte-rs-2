import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description"/>`);
export default function App($$anchor) {
	let count = 0;
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.set_attribute(meta, "content", count);
		$.append($$anchor, meta);
	});
}
