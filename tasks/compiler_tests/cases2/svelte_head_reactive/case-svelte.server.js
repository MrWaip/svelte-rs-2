import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<meta name="description"${$.attr("content", count)}/>`);
	});
}
