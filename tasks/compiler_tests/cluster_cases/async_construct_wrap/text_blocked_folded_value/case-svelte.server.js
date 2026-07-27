import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	async function getDouble(value) {
		return value * 2;
	}
	var double;
	var $$promises = $$renderer.run([async () => double = await $.async_derived(() => getDouble(count))]);
	$$renderer.push(`<button>inc</button> <p>Count: `);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(count)));
	$$renderer.push(` Double: `);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(double())));
	$$renderer.push(`</p>`);
}
