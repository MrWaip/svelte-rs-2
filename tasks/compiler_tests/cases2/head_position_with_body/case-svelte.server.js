import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = "Page";
	function handleClick() {}
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>Page</title>`);
		});
	});
	$$renderer.push(`<p>Content: Page</p>`);
}
