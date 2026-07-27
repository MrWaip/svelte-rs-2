import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	async function* gen() {
		yield 1;
	}
	for await (const n of gen()) {
		count = n;
	}
	$$renderer.push(`<p>${$.escape(count)}</p> <button>inc</button>`);
}
