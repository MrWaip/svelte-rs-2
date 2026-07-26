import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	var first, second;
	var $$promises = $$renderer.run([async () => ({first, second} = await Promise.resolve({
		first: gate,
		second: 2
	}))]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(first)));
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(second)));
	$$renderer.push(`</p>`);
}
