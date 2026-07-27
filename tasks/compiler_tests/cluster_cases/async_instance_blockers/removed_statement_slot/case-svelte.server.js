import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	var loaded, after;
	var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => after = gate + 1]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(after)));
	$$renderer.push(`</p>`);
}
