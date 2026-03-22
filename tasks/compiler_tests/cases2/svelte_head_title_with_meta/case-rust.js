import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="Benchmark component"/> <link rel="canonical" href="/benchmark"/>`, 1);
export default function App($$anchor) {
	let title = $.state("Home");
	$.set(title, "Other");
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root_1();
		$.next(2);
		$.deferred_template_effect(() => {
			$.document.title = `${$.get(title) ?? ""} - Benchmark`;
		});
		$.append($$anchor, fragment);
	});
}
