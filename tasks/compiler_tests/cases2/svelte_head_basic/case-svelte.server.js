import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<meta name="description" content="A great app"/> <link rel="icon" href="/favicon.ico"/>`);
	});
}
