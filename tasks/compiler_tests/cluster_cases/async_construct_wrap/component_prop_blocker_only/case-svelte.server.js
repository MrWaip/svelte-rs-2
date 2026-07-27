import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded;
	var $$promises = $$renderer.run([async () => loaded = await delay(1)]);
	$$renderer.async_block([$$promises[0]], ($$renderer) => {
		Child($$renderer, { value: loaded });
	});
}
