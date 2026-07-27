import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let gate = 0;
		let shown = 1;
		var loaded;
		var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => void void 0]);
		$$renderer.push(`<button>inc</button> <p>${$.escape(shown)}</p>`);
	});
}
