import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	var loaded, first, second;
	var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => {
		first = gate + 1;
		second = gate + 2;
	}]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(first)));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(second)));
	$$renderer.push(`</p>`);
}
