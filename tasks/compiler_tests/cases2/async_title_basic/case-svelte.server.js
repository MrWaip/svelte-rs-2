import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = "/api";
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>`);
			$$renderer.push(async () => $.escape((await $.save(fetch(title)))()));
			$$renderer.push(`</title>`);
		});
	});
}
