import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="test"/>`);
export default function App($$anchor) {
	let pageTitle = $.state("Home");
	let section = $.state("Dashboard");
	$.set(pageTitle, "Other");
	$.set(section, "Settings");
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.deferred_template_effect(() => {
			$.document.title = `Static - ${$.get(pageTitle) ?? ""} - App ${$.get(section) ?? ""}`;
		});
		$.append($$anchor, meta);
	});
}
