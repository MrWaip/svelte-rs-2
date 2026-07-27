import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	let counter = 1;
	var loaded;
	var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => void counter++]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(counter)));
	$$renderer.push(`</p> <p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
	$$renderer.push(`</p>`);
}
