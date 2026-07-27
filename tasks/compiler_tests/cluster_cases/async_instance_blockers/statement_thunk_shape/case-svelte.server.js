import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	let sink = 0;
	var loaded, single;
	var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => {
		void (sink = 1);
		void (sink = 2);
		single = gate + 1;
	}]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(sink)));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(single)));
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
	$$renderer.push(`</p>`);
}
