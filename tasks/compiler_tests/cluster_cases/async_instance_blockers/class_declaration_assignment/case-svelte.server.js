import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let gate = 0;
		var loaded, Box, box;
		var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate), () => {
			Box = class Box {
				value = 1;
			};
			box = new Box();
		}]);
		$$renderer.push(`<button>inc</button> <p>`);
		$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
		$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(box.value)));
		$$renderer.push(`</p>`);
	});
}
