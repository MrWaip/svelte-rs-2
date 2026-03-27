import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta charset="utf-8"/> <meta name="description" content="A page"/> <link rel="stylesheet" href="/styles.css"/>`, 1);
export default function App($$anchor) {
	let section = $.state("Dashboard");
	$.set(section, "Settings");
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root_1();
		$.next(4);
		$.deferred_template_effect(() => {
			$.document.title = `App - ${$.get(section) ?? ""}`;
		});
		$.append($$anchor, fragment);
	});
}
