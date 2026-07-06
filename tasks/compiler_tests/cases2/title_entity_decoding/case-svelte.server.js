import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "Tom";
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>&amp; Tom &lt;</title>`);
		});
	});
}
