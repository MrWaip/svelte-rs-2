import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	function row($$renderer) {
		$$renderer.push(`<p>`);
		$$renderer.push(async () => $.escape((await $.save(compute(count)))()));
		$$renderer.push(`</p>`);
	}
	row($$renderer);
	$$renderer.push(`<!----> <button>inc</button>`);
}
