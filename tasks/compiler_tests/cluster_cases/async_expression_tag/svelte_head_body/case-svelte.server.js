import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(async () => $.escape(await delay(x)));
	});
	$$renderer.push(`<button>inc</button>`);
}
