import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	let sink = 0;
	var $$promises = $$renderer.run([() => gate, () => void (sink = gate + 1)]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(sink)));
	$$renderer.push(`</p>`);
}
