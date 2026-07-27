import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>`);
			$$renderer.push(async () => $.escape((await $.save(delay(x)))()));
			$$renderer.push(`</title>`);
		});
	});
	$$renderer.push(`<button>inc</button>`);
}
