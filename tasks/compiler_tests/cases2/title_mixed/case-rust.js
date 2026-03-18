import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let section = $.state("Dashboard");
	$.set(section, "Settings");
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(() => {
			$.document.title = `App - ${$.get(section) ?? ""}`;
		});
	});
}
