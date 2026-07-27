import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	if (x >= 0) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>`);
		$$renderer.push(async () => $.escape((await $.save(delay(x)))()));
		$$renderer.push(`</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
