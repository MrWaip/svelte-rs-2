import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="A"/>`);
export default function App($$anchor) {
	$.next();
	var text = $.text("A");
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.append($$anchor, meta);
	});
	$.append($$anchor, text);
}
