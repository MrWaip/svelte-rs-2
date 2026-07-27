import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	var delayed, doubled;
	var $$promises = $$renderer.run([async () => delayed = await $.async_derived(() => Promise.resolve(count)), () => doubled = $.derived(() => count * 2)]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(delayed())));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(doubled !== doubled())));
	$$renderer.push(`</p>`);
}
