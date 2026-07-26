import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child_block(async ($$renderer) => {
		const $$0 = (await $.save(delay(x)))();
		Child($$renderer, { value: $$0 });
	});
}
