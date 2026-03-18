import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let pageTitle = $.state("Home");
	$.set(pageTitle, "Other");
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(() => {
			$.document.title = $.get(pageTitle) ?? "";
		});
	});
}
