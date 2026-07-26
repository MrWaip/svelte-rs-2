import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded, x;
	var $$promises = $$renderer.run([async () => loaded = await delay(1), () => x = 0]);
	$$renderer.push(`<button>inc</button> `);
	$$renderer.async_block([
		$$promises[0],
		$$promises[0],
		$$promises[1]
	], async ($$renderer) => {
		const $$0 = (await $.save(delay(x)))();
		Child($$renderer, {
			a: loaded,
			b: $$0
		});
	});
}
