import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let title = "Page";
	function handleClick() {}
	var p = root();
	$.head("q2w0q4", ($$anchor) => {
		$.effect(() => {
			$.document.title = "Page";
		});
	});
	$.event("click", $.document.body, handleClick);
	p.textContent = "Content: Page";
	$.append($$anchor, p);
}
