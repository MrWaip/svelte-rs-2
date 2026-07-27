import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = true;
	var loaded;
	var $$promises = $$renderer.run([async () => loaded = await $.async_derived(() => gate)]);
	$$renderer.push(`<button>toggle</button> `);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		if (gate) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`yes`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
	});
	$$renderer.push(`<!--]--> <p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(loaded())));
	$$renderer.push(`</p>`);
}
