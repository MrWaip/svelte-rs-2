import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	let before = 1;
	var loaded;
	var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate)]);
	$$renderer.push(`<button>inc</button> <p>1</p>`);
}
