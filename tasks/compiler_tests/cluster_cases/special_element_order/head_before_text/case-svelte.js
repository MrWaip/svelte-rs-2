import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<meta name="description" content="A"/>`);
export default function App($$anchor) {
	$.next();
	var text = $.text("A");
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.append($$anchor, meta);
	});
	$.append($$anchor, text);
}
