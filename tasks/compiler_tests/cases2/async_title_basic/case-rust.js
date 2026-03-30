import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let title = "/api";
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(($0) => {
			$.document.title = $0 ?? "";
		}, void 0, [() => fetch(title)]);
	});
}
