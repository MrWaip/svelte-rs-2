import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="A great app"/> <link rel="icon" href="/favicon.ico"/>`, 1);
export default function App($$anchor) {
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root_1();
		$.next(2);
		$.append($$anchor, fragment);
	});
}
