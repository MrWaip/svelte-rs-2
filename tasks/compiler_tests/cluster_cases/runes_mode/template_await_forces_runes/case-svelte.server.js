import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	$$renderer.push(`<p>`);
	$$renderer.push(async () => $.escape((await $.save(compute(count)))()));
	$$renderer.push(`</p> <button>inc</button>`);
}
