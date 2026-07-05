import * as $ from "svelte/internal/client";
var root = $.from_html(`<meta name="x" content="y"/> <link rel="canonical" href="/"/>`, 1);
export default function App($$anchor, $$props) {
	let title = $.prop($$props, "title", 3, "x");
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root();
		$.next(2);
		$.deferred_template_effect(() => {
			$.document.title = title() ?? "";
		});
		$.append($$anchor, fragment);
	});
}
