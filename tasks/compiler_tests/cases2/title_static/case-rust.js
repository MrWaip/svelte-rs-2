import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	$.head("q2w0q4", ($$anchor) => {
		$.effect(() => {
			$.document.title = "My Page";
		});
	});
}
